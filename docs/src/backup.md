# Backup

`arcadia-backup` makes incremental, versioned and encrypted backups of an Arcadia instance, and
restores them. It runs on a **backup host**, a machine other than the one running Arcadia, and
reaches the **Arcadia host** over ssh.

- **Incremental**: [restic](https://restic.net) splits the data in chunks and only stores the new
  ones. A backup after a quiet day stores a few megabytes, whatever the size of the instance.
- **Versioned**: every run makes a snapshot. Any snapshot kept by the retention policy can be
  restored, e.g. the one made 3 backups ago.
- **Only the necessary data over the network**: restic runs on the Arcadia host and only uploads
  the chunks the repository does not have yet.
- **Append only**: the Arcadia host can add snapshots, never delete or change the existing ones.
  Old snapshots are only removed by the backup host. A compromised Arcadia host can still read
  all the backups (it gets the repository password during a run) and add bogus snapshots. The
  tool detects the snapshots it did not make during a run, or dated in the future, and then
  skips the retention policy (the backup itself is kept and the run fails with an explanation),
  so that they cannot push out the real history.

## How it works

```
backup host                                      Arcadia host
───────────                                      ────────────
arcadia-backup ──── ssh, with a tunnel ─────────▶ sshd
  │                                                 │ dumps postgres, redis, mariadb
  │                                                 │ into a staging directory
  ├─ rest-server --append-only ◀──── tunnel ────────┤ restic backup (docker image or binary)
  │    (127.0.0.1 only)                             │
  └─ restic forget --prune, restic check
     (locally, on the repository)
```

Nothing listens on the network: the rest-server only listens on 127.0.0.1 of the backup host, and
the Arcadia host reaches it through the ssh connection the backup host opened. The databases are
dumped while they run, there is no downtime.

## What is backed up

| Component | Docker setup | Standard setup |
| --- | --- | --- |
| `config` | `config.yml`, `compose.override.yml`, `.env`, `ergo/ergo-conf.yaml`, `ergo/ergo.motd`, `kiwiirc/config.json` | same |
| `custom_content` | `frontend/public/home/`, `favicon.ico`, `default_user_avatar.png`, `bonus_points_icon.png`, `custom_unauth.css`, `custom_unauth.js` (in `frontend/public`) | same |
| `postgres` | `pg_dump` in the `db` service | `pg_dump` with the credentials of `config.yml` |
| `redis` | `redis-cli --rdb` in the `redis` service | `redis-cli --rdb` |
| `chevereto` | database of `chevereto_database` and the `chevereto_storage` volume, when running | the `standard.chevereto` section |
| `ergo` | database of `ergo_database` and the `ergo_data` volume, when running | the `standard.ergo` section |
| `extra` | the paths and volumes of the `extra` section | the paths of the `extra` section |

Missing optional files are skipped. Telemetry data is not backed up.

In the docker setup, `chevereto` and `ergo` are skipped when their database service was never
created. If it exists but is stopped, the backup fails: start it, or remove the service. Every
path and volume of the `extra` section must exist, or the backup fails. An extra path cannot
contain or be inside `arcadia.dir` or the work directory (`arcadia.work_dir`).

## Prerequisites

**Backup host**

- `arcadia-backup`, built from a checkout of this repository with a Rust toolchain
  (the version is pinned by `rust-toolchain`, rustup installs it automatically):
  `cargo install --path tools/arcadia-backup`. The binary lands in `~/.cargo/bin/`.
- [restic](https://restic.readthedocs.io/en/stable/020_installation.html) 0.17 or later
  (requires environment variables `RESTIC_REST_USERNAME`/`RESTIC_REST_PASSWORD` and JSON output).
  The restic of the Arcadia host (standard setup, `restic.runner: binary`) needs 0.17 or later as well.
- [rest-server](https://github.com/restic/rest-server/releases).
- An ssh client and enough disk space for the repository.

**Arcadia host**

- An ssh server with `AllowTcpForwarding yes` (at least remote forwarding for the backup user), and
  `flock` (part of util-linux).
- Docker setup: nothing else, restic runs from its docker image.
- Standard setup: `restic`, `pg_dump`/`psql`, `redis-cli`, and `mariadb-dump`/`mariadb` if chevereto
  or ergo are backed up.

## Setup

1. On the Arcadia host, create a user for the backups and give it access to the Arcadia checkout.
   In the docker setup, it has to be in the `docker` group. **Being in the docker group is
   equivalent to being root on the host**: use a dedicated key, and restrict what can log in with it.
   If you harden the key in `authorized_keys`, keep port forwarding allowed: the backup host
   opens a remote forwarding (`-R`) so that restic reaches the repository. The `restrict` option
   disables it, so use `restrict,port-forwarding ssh-ed25519 AAAA…` (or only
   `no-pty,no-agent-forwarding,no-X11-forwarding`). Likewise, sshd must not set
   `AllowTcpForwarding no` for that user.

   ```bash
   sudo useradd --create-home arcadia-backup
   sudo usermod -aG docker arcadia-backup
   ```

2. On the backup host, create a key and authorize it on the Arcadia host:

   ```bash
   ssh-keygen -t ed25519 -f ~/.ssh/arcadia-backup -N ""
   ssh-copy-id -i ~/.ssh/arcadia-backup.pub arcadia-backup@arcadia.example.com
   ```

3. On the backup host, add the Arcadia host to `~/.ssh/config` so ssh uses the key without
   prompting:

   ```
   Host arcadia
     HostName arcadia.example.com
     User arcadia-backup
     IdentityFile ~/.ssh/arcadia-backup
     IdentitiesOnly yes
   ```

   Then test with batch mode (which arcadia-backup uses):

   ```bash
   ssh -o BatchMode=yes arcadia true
   ```

   The host key was already accepted by `ssh-copy-id` above. If the test fails on the host key,
   run `ssh arcadia` once interactively to accept it. Then set `ssh.target: arcadia` in the
   configuration.

4. Create the password of the repository. **Without this password, the backups cannot be read:
   keep a copy of it somewhere else than on the backup host.**

   ```bash
   mkdir -p ~/.config/arcadia-backup
   openssl rand -base64 32 > ~/.config/arcadia-backup/password
   chmod 600 ~/.config/arcadia-backup/password
   ```

5. Copy [`tools/arcadia-backup/arcadia-backup.example.yml`](https://github.com/Arcadia-Solutions/arcadia/blob/main/tools/arcadia-backup/arcadia-backup.example.yml)
   to `~/.config/arcadia-backup/arcadia-backup.yml` and fill it in. Every key is documented there.

6. Create the repository and make a first backup:

   ```bash
   arcadia-backup init
   arcadia-backup backup
   ```

## Scheduling

With systemd on the backup host, adjust the binary path (`~/.cargo/bin/arcadia-backup` after
`cargo install`, or wherever you copied it). For `restic.binary` and
`restic.rest_server`, use absolute paths or set `PATH` in the service unit; systemd user units
have a minimal `PATH` that may not include them.

`~/.config/systemd/user/arcadia-backup.service`:

```ini
[Unit]
Description=Arcadia backup

[Service]
Type=oneshot
ExecStart=/path/to/arcadia-backup backup
```

A `oneshot` unit has no start timeout by default, so a long first backup is not cut short. The
flip side is that a hung run keeps the unit active: check it with
`systemctl --user status arcadia-backup` and the journal. Set `TimeoutStartSec=` only
deliberately, to a value above the longest run you expect, because a killed run stops the backup
half way.

and `~/.config/systemd/user/arcadia-backup.timer`:

```ini
[Unit]
Description=Daily Arcadia backup

[Timer]
OnCalendar=*-*-* 03:00:00
RandomizedDelaySec=15m
Persistent=true

[Install]
WantedBy=timers.target
```

```bash
systemctl --user daemon-reload
systemctl --user enable --now arcadia-backup.timer
loginctl enable-linger "$USER"   # run the timer without an open session
journalctl --user -u arcadia-backup   # logs
```

Or with cron: `0 3 * * * /path/to/arcadia-backup backup >> ~/arcadia-backup.log 2>&1`.

Two runs never overlap: a run holds a lock on the Arcadia host, released even if it crashes.

## Snapshots and retention

```bash
arcadia-backup snapshots
```

```
  #  id        date                       components
  0  4bba301e  2026-09-30 03:00:12 +0000  config, custom_content, postgres, redis, chevereto
  1  9a1c07d2  2026-09-29 03:00:09 +0000  config, custom_content, postgres, redis, chevereto
```

After each backup, the `retention` policy is applied (`restic forget --prune`) and, every
`check.every_n_backups` backups, the repository is checked. Both can also be run by hand:
`arcadia-backup prune`, `arcadia-backup check [--read-data-subset 10%]`.

The retention policy is skipped, with an error, when a run finds a snapshot it did not make or one
dated more than an hour in the future (see [Append only](#backup)). `prune` refuses likewise for
snapshots from the future. Inspect the snapshots, and remove the bogus ones with `restic forget`
on the repository.

## Reading a backup

```bash
arcadia-backup extract --snapshot <latest|N|id> --target <dir>
```

Extracts a snapshot, unencrypted, into `<dir>`, which must be new or empty (it is created with
mode `0700`). The command prints where each item of every component ended up, e.g.
`postgres/postgres.sql: <dir>/arcadia/postgres/postgres.sql`. In docker mode the files are under
`<dir>/arcadia/<component>/...`; in standard mode they keep their host paths under `<dir>`.

It only runs on the backup host, against the repository: it does not contact the Arcadia host.

The extracted files hold secrets (`config.yml`, database dumps) and are not encrypted: delete the
directory when you are done.

Plain restic works too, with `RESTIC_REPOSITORY` and `RESTIC_PASSWORD_FILE` set: `restic dump
<snapshot> <path>` prints one file, `restic mount <dir>` browses the snapshots.

## Restore

A restore **replaces** the current data of the Arcadia host with the one of the snapshot.

```bash
arcadia-backup restore                    # the latest snapshot
arcadia-backup restore --snapshot 3       # 3 backups before the latest
arcadia-backup restore --snapshot 4bba301e
arcadia-backup restore --only postgres,redis
arcadia-backup restore --skip-config      # keep the current configuration files
```

It shows what it is going to overwrite and asks for confirmation (`--yes` skips it). It warns when
the snapshot was made with another commit of Arcadia than the one checked out on the host: the
database schema may not match the code.

In the docker setup, it stops the application services (backend, tracker, frontend, chevereto_php,
ergo, kiwiirc), restores, and starts them again with `docker compose up -d --build` (the frontend
image embeds the configuration and the custom content, so it is rebuilt).

In the standard setup, stop the Arcadia services **and redis** before confirming, and start them
again afterwards. The user running the restore must be able to write the restored directories and
to drop and create the databases. The configuration must have the `standard` sections of the
components in the snapshot (`chevereto`, `ergo`, `redis`), and the password files must be readable
on the Arcadia host: this is checked before anything is modified.

If redis has AOF enabled (`appendonly yes`), it loads the append only files and ignores the
restored `dump.rdb`: disable AOF, or remove the AOF files (the `appendonlydir` directory next to
`dump.rdb`), before starting redis again. The docker setup does it itself.

### Restoring on a new host

1. Install Docker (or the standard setup prerequisites) and create the backup user, as above.
2. Clone Arcadia at the commit the snapshot was made with (`arcadia-backup restore` prints it when it
   differs, it is also in the snapshot's `meta.json`), in the directory set as `arcadia.dir`.
3. Point `ssh.target` to the new host, then `arcadia-backup restore`. The configuration files are
   restored first, then the data.
4. Start the stack: `docker compose --profile full up -d --build` (with the profiles you use).

## Keeping a copy elsewhere

The repository is a single directory on the backup host. To keep a second copy, e.g. on S3 or
Backblaze B2, use `restic copy` from the backup host.

1. Create a password for the secondary copy, and keep it safe as well:

   ```bash
   openssl rand -base64 32 > ~/.config/arcadia-backup/password-secondary
   chmod 600 ~/.config/arcadia-backup/password-secondary
   ```

   **Without this password, the secondary copy cannot be read: keep a copy somewhere else.**

2. Initialize the secondary repository and copy the data:

   ```bash
   export AWS_ACCESS_KEY_ID=...
   export AWS_SECRET_ACCESS_KEY=...

   restic -r s3:s3.amazonaws.com/my-bucket init \
     --from-repo /srv/backups/arcadia \
     --from-password-file ~/.config/arcadia-backup/password \
     --password-file ~/.config/arcadia-backup/password-secondary \
     --copy-chunker-params

   restic -r s3:s3.amazonaws.com/my-bucket copy \
     --from-repo /srv/backups/arcadia \
     --from-password-file ~/.config/arcadia-backup/password \
     --password-file ~/.config/arcadia-backup/password-secondary
   ```

3. The secondary copy has its own retention policy. Run it regularly (daily, e.g. from cron),
   adapting the keep flags to your needs:

   ```bash
   export AWS_ACCESS_KEY_ID=...
   export AWS_SECRET_ACCESS_KEY=...

   restic -r s3:s3.amazonaws.com/my-bucket forget --prune \
     --password-file ~/.config/arcadia-backup/password-secondary \
     --host arcadia.example.com \
     --tag arcadia \
     --group-by host \
     --keep-daily 7 \
     --keep-weekly 4 \
     --keep-monthly 12
   ```

   `--host` is the host recorded in the snapshots: `arcadia.name` of the configuration, or the
   host of `ssh.target` (without the user) when it is not set. `arcadia-backup snapshots` or
   `restic snapshots` shows it. Without regular pruning, the copy grows forever.

## Troubleshooting

- **`cannot connect to … over ssh`**: try `ssh <target>` by hand. Authentication must work without a
  prompt (key, agent, `~/.ssh/config`).
- **`another arcadia-backup run holds …/lock`**: a run is in progress. The lock is released when that
  run ends, even if it crashes.
- **`is not a restic repository, run arcadia-backup init first`**: `restic.repository` does not point
  to a repository.
- **A component fails**: nothing is stored for that run, the error names the component and the remote
  error is printed just above it. Fix it and run again.
- **Restore failed half way**: it prints what was restored; the services are left stopped. Fix the
  cause and restore again, a restore can be repeated.
