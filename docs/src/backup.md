# Backup

Backups are made with [restic](https://restic.net): every run is an encrypted, deduplicated snapshot,
so only what changed is stored and transferred.

- `backup/backup.sh` runs on the arcadia server. It dumps the databases (arcadia, ergo, chevereto), then
  snapshots the dumps, the configuration files (`config.yml`, `ergo/ergo-conf.yaml`, `ergo/ergo.motd`,
  `kiwiirc/config.json`, `compose.override.yml`) and the volumes (docker) or data directories (bare
  metal) into a local repository, and prunes it.
- `backup/cron.sh` runs `backup.sh` on the schedule of `backup.cron`, inside the `backup_cron` compose
  service (docker mode only).
- `backup/pull.sh` runs on a separate backup host. It copies the snapshots it does not have yet over
  ssh, so the backup host keeps its own history, which the arcadia server cannot reach or delete.
- `backup/restore.sh` restores a snapshot, on the same server or on a new one.

The database volumes are never copied raw (files of a running database are not consistent): the
dumps cover them.

## Requirements

- docker mode: nothing but docker, restic runs from its image.
- host mode: [restic](https://restic.readthedocs.io/en/stable/020_installation.html) 0.17 or newer,
  `pg_dump`/`psql`, `mariadb-dump`/`mariadb` matching the server versions, and `curl` (backup/restore.sh
  uses it to check that the backend is stopped). The scripts run as root.
- backup host: restic 0.17 or newer, a checkout of this repository (or `backup/pull.sh` and
  `scripts/config_value.sh`).

## Configuration

Everything is in the `backup` section of `config.yml`, documented in `config.example.yml`. The
database credentials are read from the `database` section, and in docker mode from the ergo and
chevereto containers.

Create the repository password, and keep a copy somewhere safe, it is not part of the backups:

```bash
head -c 32 /dev/urandom | base64 > /root/.arcadia-restic-password
chmod 600 /root/.arcadia-restic-password
```

The repository is created by the first run of `backup/backup.sh`.

`keep` decides how many snapshots the server keeps. It must outlast the longest outage of the backup
host, or snapshots are pruned before being pulled. `remote_repo` optionally copies every snapshot to
a second repository (any [restic backend](https://restic.readthedocs.io/en/stable/030_preparing_a_new_repo.html);
in docker mode a local path, or a backend configured by `AWS_ACCESS_KEY_ID`/`AWS_SECRET_ACCESS_KEY`).

## Scheduling

`backup.mode` decides how: in docker mode the `backup_cron` service runs the job on the schedule of
`backup.cron`, in host mode (or if you prefer the host's own tools) a systemd timer runs it.

### Docker: the backup_cron service

```bash
docker compose --profile backup up -d backup_cron
```

`backup.cron` holds the schedule in cron syntax, read in `backup.cron_timezone` (containers have no
timezone of their own). The job is `backup/backup.sh` itself, so everything the sections above
describe applies. It runs as root, which is why the service mounts the docker socket: the job drives
`docker compose exec` and runs restic from its image.

Each run gets a container of its own, started by `backup/cron.sh` with what `backup.sh` needs at the
paths of the `backup` section: the repository (read only), the docker socket, `backup.dump_dir` and
`backup.password_file`. `config.yml` is therefore the only configuration, and compose reads nothing
from the environment for this service.

The output of a run, and the fact that it failed, are in the container log:

```bash
docker compose logs backup_cron
```

### systemd, on the host

`/etc/systemd/system/arcadia-backup.service`:

```ini
[Unit]
Description=Arcadia backup

[Service]
Type=oneshot
ExecStart=/opt/arcadia/backup/backup.sh
```

`/etc/systemd/system/arcadia-backup.timer`:

```ini
[Timer]
OnCalendar=*-*-* 03:00
Persistent=true

[Install]
WantedBy=timers.target
```

```bash
systemctl enable --now arcadia-backup.timer
```

A failing run exits with a non-zero code: use `OnFailure=` (or cron's mail) to be notified.

## Backup host

On the arcadia server, give the backup host's ssh key access to a `backup` user that can read the
repository and write its locks (restic creates new files group readable when the repository is):

```bash
useradd -m backup
chgrp -R backup /var/backups/arcadia/repo
chmod -R g+rX /var/backups/arcadia/repo
chmod g+w /var/backups/arcadia/repo/locks
find /var/backups/arcadia/repo -type d -exec chmod g+s {} +
```

On the backup host, write a configuration file, e.g. `/etc/arcadia-backup-pull.yml`:

```yaml
backup_pull:
  ssh: backup@arcadia.example.com
  source_repo: /var/backups/arcadia/repo
  # command run on the arcadia server before pulling, empty when it runs its own timer, e.g.
  # sudo /opt/arcadia/backup/backup.sh (needs a sudo rule for the backup user)
  trigger: ""
  repo: /srv/backups/arcadia
  # a copy of the arcadia repository password
  password_file: /root/.arcadia-restic-password
  # retention of the backup host, longer than the server's and keeping at least what it keeps
  keep: --keep-daily 30 --keep-weekly 12 --keep-monthly 12
```

and schedule `ARCADIA_CONFIG=/etc/arcadia-backup-pull.yml /opt/arcadia/backup/pull.sh` after the
server's backup, the same way as above (`Environment=ARCADIA_CONFIG=…` in the service). It fails when
no new snapshot was found, which also catches a server that stopped backing up. Missed days are
caught up: every missing snapshot is copied. The backup host's `keep` must keep at least every
snapshot the server still keeps, otherwise pruned snapshots are copied again and counted as new.

## Restore

```bash
backup/restore.sh            # latest snapshot
backup/restore.sh 1a2b3c4d   # a given snapshot, ids from `restic snapshots`
backup/restore.sh latest -y  # no confirmation
```

Stop the backup timer (and the backup host's trigger) before restoring, otherwise a backup can run in
the middle and `latest` changes.

It restores the configuration files, the volumes (or data directories) and the databases. In docker
mode it stops the running services first and starts them again at the end. In host mode, stop
arcadia, ergo and redis yourself first, and create the postgres role of the `database` section on a
new server.

### On a new server

1. Clone the repository, `cp config.example.yml config.yml`: only the `backup` section matters, it is
   overwritten by the restored `config.yml`. `dump_dir` must be the one used for the backups.
2. Put the password file in place.
3. From the backup host, copy the repository to the new server:
   ```bash
   restic -r sftp:root@new-server:/var/backups/arcadia/repo init --from-repo /srv/backups/arcadia --copy-chunker-params
   restic -r sftp:root@new-server:/var/backups/arcadia/repo copy --from-repo /srv/backups/arcadia
   ```
4. `backup/restore.sh`, then start the services.

### From docker to bare metal (or back)

Not scripted. Restore a volume into a directory with
`restic restore latest:/data/arcadia_ergo_data --target /var/lib/ergo`, and load the dumps of
`dump_dir` with `psql` and `mariadb`.

## Limits

- Paths and volume names cannot contain spaces, the crontab of the `backup_cron` service included: the
  job it schedules is one unquoted line.
- Restoring in docker mode creates the volumes outside of docker compose, which warns that they were
  "not created by Docker Compose"; it is harmless.
