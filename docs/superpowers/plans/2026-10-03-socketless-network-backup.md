# Socketless Network Backup Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make docker-mode backups run with no docker socket, no docker CLI, and no published DB ports — dumping over the shared compose network and running restic inside the backup container.

**Architecture:** `backup_cron` joins the default compose network and dumps `db`/`*_database` by service name over TCP; restic runs in-container against bind-mounted volumes and a single backup dir. The docker/host branches in `backup.sh`/`lib.sh` collapse (postgres dump becomes one command; restic always runs locally). `cron.sh` drops its nested-docker-run machinery. Host mode and `restore.sh` are untouched.

**Tech Stack:** bash/POSIX sh, docker compose, restic, pg_dump (postgres:18-alpine), mariadb-client, busybox crond, GitHub Actions.

**Spec:** `docs/superpowers/specs/2026-10-03-backup-socketless-network-design.md`

## Global Constraints

- **Never `git push`.** Local commits only (user rule).
- Every commit message ends with: `Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>`.
- Backup image base is `postgres:18-alpine`: its `pg_dump` **major must match the `db` service** (`postgres:18.0`) — bump both together. The rest (`bash restic mariadb-client tzdata`) comes from `apk`.
- restic in the container (alpine package) and on the host/CI (pinned `0.19.1` binary) share one repo — both must stay restic **≥ 0.17** (repo format v2).
- mariadb dumps are **logical only** (`--single-transaction`, no binary/replication flags), so alpine's `mariadb-client` dumping `mariadb:jammy` stays compatible.
- No DB ports are published. Docker-mode `backup.sh` runs **only inside a container on the compose network** — scheduled via `backup_cron`, manual via `docker compose --profile backup run --rm --entrypoint /arcadia/backup/backup.sh backup_cron`.
- In docker mode, every **local-path** target (`backup.repo`, `backup.dump_dir`, a local-path `backup.remote_repo`) must live under the single `${BACKUP_DIR}` bind mount so restic can see it in-container. Backend-URL targets (`s3:`/`rest:`/`sftp:`) are exempt.
- `backup.volumes` lists **unprefixed** volume names (`chevereto_storage ergo_data redis_data`); compose mounts each at `/data/<name>` so paths are project-name-independent.

## Review Focus

- **Empty/failed mariadb dump must never replace a good dump** — restore does DROP+CREATE, so an empty `.sql` wipes the DB. Dump to a temp file; only move into place on success **and** non-empty. (Task 3)
- **Local-path `remote_repo` not under `${BACKUP_DIR}`** — `restic copy` fails in-container because the path isn't mounted. Doc the constraint; test with a path under the mount. (Task 3)
- **Env-override collision** — a pre-existing env var matching `SECTION_KEY` silently shadows `config.yml`. Test that the override fires only when set, and the YAML wins when unset. (Task 1)
- **Scheduled job's environment** — busybox crond runs with a minimal env; restic/pg_dump must be on `PATH` and the mariadb creds present, or the cron backup fails silently. Emit `PATH=` and the cred vars as crontab env lines; assert the scheduled run produces a snapshot with the dumps. (Task 3)
- **postgres password auth over the network** — docker-mode dump now authenticates over TCP; a wrong `database.password`/pg_hba makes it fail. Covered by `backup/test.sh docker` dumping postgres over the network. (Task 3)

---

### Task 1: Env-override layer in `config_value.sh`

**Files:**
- Modify: `scripts/config_value.sh`
- Test: `scripts/config_value_test.sh` (create)

**Interfaces:**
- Produces: `config_value <section> <key>` — unchanged signature; now returns the env var `<SECTION>_<KEY>` (upper-cased, `-`→`_`) first when it is set and non-empty, else the YAML value.

- [ ] **Step 1: Write the failing test**

Create `scripts/config_value_test.sh`:

```sh
#!/usr/bin/env bash
# Self-check for config_value.sh: env override wins when set, YAML otherwise.
set -eu
cd "$(dirname "$0")"
. ./config_value.sh
T=$(mktemp)
trap 'rm -f "$T"' EXIT
printf 'backup:\n  repo: /from/yaml\n' > "$T"
export ARCADIA_CONFIG="$T"

[ "$(config_value backup repo)" = /from/yaml ] || { echo "FAIL: yaml value"; exit 1; }
BACKUP_REPO=/from/env; export BACKUP_REPO
[ "$(config_value backup repo)" = /from/env ] || { echo "FAIL: env override"; exit 1; }
BACKUP_REPO="" ; [ "$(config_value backup repo)" = /from/yaml ] || { echo "FAIL: empty env ignored"; exit 1; }
unset BACKUP_REPO
# an unrelated key is not shadowed by an unrelated env var
[ -z "$(config_value backup missing)" ] || { echo "FAIL: missing key"; exit 1; }
echo "config_value_test passed"
```

- [ ] **Step 2: Run it, verify it fails**

Run: `bash scripts/config_value_test.sh`
Expected: `FAIL: env override` (env is not consulted yet).

- [ ] **Step 3: Add the env shim to `config_value()`**

In `scripts/config_value.sh`, at the top of the function body (before the `awk`), POSIX-safe (it is sourced by busybox `sh` too — no bash-only `${!var}`):

```sh
config_value() {
    # An environment variable named <SECTION>_<KEY> (upper-cased, '-' -> '_') overrides the file,
    # so a deployment can set values without editing config.yml. Empty or unset falls through.
    _cv_env=$(printf '%s_%s' "$1" "$2" | tr '[:lower:]-' '[:upper:]_')
    eval "_cv_val=\${$_cv_env:-}"
    if [ -n "$_cv_val" ]; then printf '%s\n' "$_cv_val"; return; fi
    awk -v section="$1:" -v key="$2:" '
        ... (unchanged awk body) ...
    ' "${ARCADIA_CONFIG:-config.yml}"
}
```

Update the file's top comment to note the env override.

- [ ] **Step 4: Run it, verify it passes**

Run: `bash scripts/config_value_test.sh`
Expected: `config_value_test passed`

- [ ] **Step 5: Commit**

```bash
git add scripts/config_value.sh scripts/config_value_test.sh
git commit -m "feat(backup): let env vars override config.yml values"
```

---

### Task 2: `remote_repo` as a list of push targets

**Files:**
- Modify: `backup/lib.sh:13,22` (REMOTE → REMOTES, validation loop)
- Modify: `backup/backup.sh:53-60` (`push()` loops)
- Modify: `backup/test.sh:38,183-184` (two remotes, assert both)
- Modify: `config.example.yml` (`backup.remote_repo` comment)

**Interfaces:**
- Consumes: `cfg remote_repo` → space-separated list (zero or more entries).
- Produces: `push()` copies the local repo to **each** target.

- [ ] **Step 1: Update the test to expect two targets**

In `backup/test.sh`, change `backup_config()` line `remote_repo: $T/remote` to a two-target list and add a remote dir, then assert both receive the copy. Replace the single-remote line with:

```sh
  remote_repo: $T/remote $T/remote2
```

After the existing `count "$T/remote"` assertion (around line 184) add:

```sh
[ "$(count "$T/remote2")" = 1 ] || fail "backup.sh did not copy to the second remote_repo"
```

- [ ] **Step 2: Run host test, verify it fails**

Run: `backup/test.sh host`
Expected: FAIL `backup.sh did not copy to the second remote_repo` (only the first is copied).

- [ ] **Step 3: Make `REMOTES` a list and loop `push()`**

`backup/lib.sh`: rename and validate each entry.

```sh
REMOTES=$(cfg remote_repo)
# ...
for _r in $REMOTES; do
    case $_r in /* | *:*) ;; *) echo "each backup.remote_repo entry must be absolute or a restic backend URL, not '$_r'" >&2; exit 1 ;; esac
done
```

`backup/backup.sh`, `push()`:

```sh
push() {
    local remote
    for remote in $REMOTES; do
        restic -r "$remote" cat config > /dev/null 2>&1 ||
            restic -r "$remote" init --from-repo "$REPO" --copy-chunker-params
        restic -r "$remote" copy --from-repo "$REPO"
        # shellcheck disable=SC2046 # keep holds several flags
        restic -r "$remote" forget --host arcadia --prune $(cfg keep)
    done
}
```

Update `config.example.yml` `remote_repo` comment: "space-separated list of repositories the snapshots are copied to (local paths, s3:, rest:, …), empty = none".

- [ ] **Step 4: Run host test, verify it passes**

Run: `backup/test.sh host`
Expected: `backup test passed (host)`

- [ ] **Step 5: Commit**

```bash
git add backup/lib.sh backup/backup.sh backup/test.sh config.example.yml
git commit -m "feat(backup): allow multiple remote_repo push targets"
```

---

### Task 3: Docker-mode socketless network migration

This is the atomic core: the stack cannot be half-migrated, so all of 3a–3g land together and the task is verified by one `backup/test.sh docker`. Do the edits in order, then the single test run at Step 3h.

**Files:**
- Modify: `compose.yml` (anchors; `backup_cron` mounts/env; drop socket)
- Modify: `backup/cron.Dockerfile` (base `postgres:18-alpine`; clients, not docker-cli)
- Modify: `backup/cron.sh` (drop nested docker run; emit crontab env lines)
- Modify: `backup/backup.sh` (collapse `dump_postgres`; network `dump_mariadb`)
- Modify: `backup/lib.sh` (collapse `restic()`; drop `RESTIC_IMAGE`; drop volume-inspect)
- Modify: `config.example.yml` (`backup.volumes` unprefixed; `mode` comment)
- Create: `.env.example` (document `BACKUP_DIR`, `RESTIC_PASSWORD_FILE`)
- Modify: `backup/test.sh` (docker-mode paths, in-container invocation, crontab assertion)

**Interfaces:**
- Consumes (from compose env, docker mode): `ERGO_DB_USER/PASSWORD/NAME`, `CHEVERETO_DB_USER/PASSWORD/NAME`.
- Produces: `data_targets()` emits `/data/<unprefixed-volume>`; `restic()` always runs `command restic` locally.

#### 3a — `compose.yml`

- [ ] **Step 1: Add scalar anchors and reference them in the mariadb services**

At the top of the file (above `services:`), add shareable scalars:

```yaml
x-ergo-db-name: &ergo-db-name ergo_history
x-ergo-db-user: &ergo-db-user ergo
x-ergo-db-pass: &ergo-db-pass ergo_password
x-chevereto-db-name: &chevereto-db-name chevereto
x-chevereto-db-user: &chevereto-db-user chevereto
x-chevereto-db-pass: &chevereto-db-pass arcadia
```

In `ergo_database.environment`, replace the literals with the anchors (keep `MYSQL_ROOT_PASSWORD: password`):

```yaml
      MYSQL_DATABASE: *ergo-db-name
      MYSQL_USER: *ergo-db-user
      MYSQL_PASSWORD: *ergo-db-pass
```

Do the same for `chevereto_database.environment` with the `*chevereto-db-*` anchors.

- [ ] **Step 2: Rewrite the `backup_cron` service — drop the socket, join the network via mounts/env**

```yaml
  backup_cron:
    profiles: [backup]
    build:
      context: ./
      dockerfile: backup/cron.Dockerfile
    restart: always
    environment:
      ERGO_DB_NAME: *ergo-db-name
      ERGO_DB_USER: *ergo-db-user
      ERGO_DB_PASSWORD: *ergo-db-pass
      CHEVERETO_DB_NAME: *chevereto-db-name
      CHEVERETO_DB_USER: *chevereto-db-user
      CHEVERETO_DB_PASSWORD: *chevereto-db-pass
    volumes:
      - .:/arcadia:ro
      - ${BACKUP_DIR:-./backups}:/var/backups/arcadia
      - ${RESTIC_PASSWORD_FILE:-./backups/password}:/var/backups/arcadia/password:ro
      - redis_data:/data/redis_data:ro
      - ergo_data:/data/ergo_data:ro
      - chevereto_storage:/data/chevereto_storage:ro
```

Update the service's comment block to say it dumps over the compose network and runs restic in-container (no socket, no docker cli).

#### 3b — `backup/cron.Dockerfile`

- [ ] **Step 3: Base on `postgres:18-alpine`, install clients not docker-cli**

```dockerfile
# The backup_cron image. Based on postgres:18-alpine so pg_dump matches the db service exactly;
# restic, the mariadb client and bash are added, tzdata for backup.cron_timezone. No docker cli:
# the job dumps over the compose network and runs restic in this container. The repository is
# mounted at /arcadia by compose.
FROM postgres:18-alpine
RUN apk add --no-cache bash restic mariadb-client tzdata
ENTRYPOINT ["/arcadia/backup/cron.sh"]
```

#### 3c — `backup/cron.sh`

- [ ] **Step 4: Collapse cron.sh — write a crontab that runs backup.sh in this container**

Replace the body after the `TZ` export (remove the `docker inspect` lines, `PROJECT`/`REPO`/`IMAGE`/`DUMP_DIR`/`PASSWORD_FILE` and the nested `docker run`). The crontab carries env lines so busybox crond's minimal environment still has `PATH`, `TZ` and the mariadb creds:

```sh
mkdir -p /etc/crontabs
{
    printf 'PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin\n'
    printf 'TZ=%s\n' "$TZ"
    for v in ERGO_DB_USER ERGO_DB_PASSWORD ERGO_DB_NAME \
             CHEVERETO_DB_USER CHEVERETO_DB_PASSWORD CHEVERETO_DB_NAME; do
        eval "val=\${$v:-}"
        [ -n "$val" ] && printf '%s=%s\n' "$v" "$val"
    done
    printf '%s /arcadia/backup/backup.sh >>/proc/1/fd/1 2>&1 || echo "backup.sh failed: $?" >>/proc/1/fd/1\n' "$CRON"
} > /etc/crontabs/root
chmod 600 /etc/crontabs/root
echo "scheduled in backup.cron: $(cat /etc/crontabs/root)"
exec crond -f -c /etc/crontabs
```

Update the top comment: docker mode only, runs backup.sh in this container, no socket.

#### 3d — `backup/backup.sh`

- [ ] **Step 5: Collapse `dump_postgres` to one network command**

In docker mode `database.host` is already the `db` service, so both modes use the same TCP dump:

```sh
dump_postgres() {
    local user name
    user=$(config_value database user)
    name=$(config_value database name)
    PGPASSWORD=$(config_value database password) pg_dump \
        -h "$(config_value database host)" -p "$(config_value database port)" \
        -U "$user" -d "$name" --no-owner --no-privileges > "$DUMP_DIR/arcadia.sql"
}
```

- [ ] **Step 6: Make `dump_mariadb` dump over the network as the per-db user (docker)**

Signature gains a third arg: the env-var prefix for the docker creds. Host branch is unchanged. The temp-file + non-empty guard is the wipe-prevention from Review Focus:

```sh
# $1: compose service, also the dump name. $2: database name (host mode). $3: env prefix for the
# per-database credentials (docker mode). Docker: dumped over the compose network as the owning
# user; an unreachable service (optional profile down) or an empty dump is skipped, never written,
# so restore cannot wipe the database from it.
dump_mariadb() {
    if [ "$MODE" = docker ]; then
        local u p d tmp
        eval "u=\${${3}_USER:-}"; eval "p=\${${3}_PASSWORD:-}"; eval "d=\${${3}_NAME:-}"
        [ -n "$u" ] && [ -n "$d" ] || return 0
        tmp=$(mktemp "$DUMP_DIR/.$1.XXXXXX")
        if MYSQL_PWD="$p" mariadb-dump -h "$1" -u "$u" --single-transaction "$d" > "$tmp" 2> /dev/null && [ -s "$tmp" ]; then
            mv "$tmp" "$DUMP_DIR/$1.sql"
        else
            rm -f "$tmp"
        fi
    elif [ -n "$2" ]; then
        mariadb-dump --single-transaction "$2" > "$DUMP_DIR/$1.sql"
    fi
}
```

Update the two call sites:

```sh
dump_mariadb ergo_database "$(cfg ergo_db)" ERGO_DB
dump_mariadb chevereto_database "$(cfg chevereto_db)" CHEVERETO_DB
```

- [ ] **Step 7: Drop the socket-only volume-inspect guard in `snapshot()`**

Remove the line `if [ "$MODE" = docker ]; then for v in $VOLUMES; do docker volume inspect "$v" > /dev/null; done; fi` (it needs the socket). The volumes are now bind-mounted by compose; a volume listed but never populated is a known corner (see backup.md limitations).

#### 3e — `backup/lib.sh`

- [ ] **Step 8: Collapse `restic()` and drop the image indirection**

Delete `RESTIC_IMAGE=restic/restic:0.19.1`. Replace `restic()` with the local-only form (it now runs in-container for docker and on the host for host mode):

```sh
# restic on the local repository, always run locally: in the backup container (docker mode) or on
# the host (host mode). dump_dir, the volumes and the repositories are visible at their config paths
# through the container's mounts or the host filesystem.
restic() {
    RESTIC_REPOSITORY="$REPO" RESTIC_PASSWORD_FILE="$PASSWORD_FILE" \
        RESTIC_FROM_PASSWORD_FILE="$PASSWORD_FILE" \
        AWS_ACCESS_KEY_ID="${AWS_ACCESS_KEY_ID:-}" AWS_SECRET_ACCESS_KEY="${AWS_SECRET_ACCESS_KEY:-}" \
        command restic --retry-lock 30m "$@"
}
```

Confirm `data_targets()` still reads `VOLUMES` and emits `/data/$t` — it does; with unprefixed `VOLUMES` and the compose mounts at `/data/<name>` the paths line up.

#### 3f — config + `.env`

- [ ] **Step 9: Unprefix `backup.volumes` and document the env/dir**

`config.example.yml`: set `volumes: chevereto_storage ergo_data redis_data` and update its comment ("unprefixed compose volume names; mounted at /data/<name> by the backup_cron service"). Update the `mode` comment: docker mode dumps over the compose network and runs restic in the backup_cron container (no docker socket).

Create `.env.example`:

```sh
# Host directory holding the restic repo and dumps for docker-mode backups (backup.repo and
# backup.dump_dir must live under /var/backups/arcadia, where this is mounted in backup_cron).
BACKUP_DIR=/var/backups/arcadia
# Host path of the restic repository password file.
RESTIC_PASSWORD_FILE=/var/backups/arcadia/password
```

#### 3g — `backup/test.sh` (docker mode)

- [ ] **Step 10: Point docker-mode config at container paths and the shared mount**

Give `backup_config()` mode-aware local paths so docker-mode targets sit under `/var/backups/arcadia` while host mode keeps `$T`. At the top of the script add:

```sh
if [ "$MODE" = docker ]; then
    export BACKUP_DIR="$T" RESTIC_PASSWORD_FILE="$T/password"
    BREPO=/var/backups/arcadia/repo; BDUMPS=/var/backups/arcadia/dumps
    BREMOTE=/var/backups/arcadia/remote; BREMOTE2=/var/backups/arcadia/remote2
    BPASS=/var/backups/arcadia/password
    VOLUMES="redis_data ergo_data chevereto_storage"
else
    BREPO="$T/repo"; BDUMPS="$T/dumps"; BREMOTE="$T/remote"; BREMOTE2="$T/remote2"
    BPASS="$T/password"
    VOLUMES="${COMPOSE_PROJECT_NAME}_chevereto_storage ${COMPOSE_PROJECT_NAME}_ergo_data ${COMPOSE_PROJECT_NAME}_redis_data"
fi
```

Rewrite `backup_config()` to use these (`repo: $1` callers pass a container-or-host path; keep the `$1` parameter but have callers pass `$BREPO`/`$T/pulled`/`$BCRONREPO`):

```sh
backup_config() { # $1: repository path (as restic sees it). $2: cron schedule
    cat <<EOF
backup:
  mode: $MODE
  cron: "$2"
  cron_timezone: Europe/Berlin
  repo: $1
  password_file: $BPASS
  dump_dir: $BDUMPS
  keep: --keep-last 20
  remote_repo: $BREMOTE $BREMOTE2
  volumes: $VOLUMES
  paths: $PATHS
  ergo_db: ergo_history
  chevereto_db: chevereto
EOF
}
```

Note: host-mode restic/`count`/`du` read `$T/...`; docker-mode they read the same bytes via the `BACKUP_DIR=$T` bind mount (`$T/repo` on the host == `/var/backups/arcadia/repo` in the container). `count`/`r` keep using the host `$T/...` paths — update the backup/remote assertions to use `$T/repo`, `$T/remote`, `$T/remote2` explicitly (host paths), independent of the in-container `$BREPO`.

- [ ] **Step 11: Run docker-mode backup inside a container, not on the host**

Add a helper and replace every `as_root backup/backup.sh` with it:

```sh
run_backup() {
    if [ "$MODE" = docker ]; then
        dc --profile backup run --rm --entrypoint /arcadia/backup/backup.sh backup_cron
    else
        as_root backup/backup.sh
    fi
}
```

Replace the four `as_root backup/backup.sh` calls (snapshot, no-change, stopped-service, catch-up) with `run_backup`. The stopped-service test (`dc stop chevereto_database` → run → assert no `chevereto_database.sql`) now exercises the network skip-on-unreachable path — keep it.

- [ ] **Step 12: Update the `backup_cron` crontab assertion**

Replace the old nested-`docker run` expectation (the `case $crontab` matching `docker run ... -v $T/dumps`) with the new in-container form, and drop the stale comment about reaching the DB "through docker compose":

```sh
    case $crontab in
        *"/arcadia/backup/backup.sh"*) ;;
        *) fail "backup_cron scheduled nothing usable: ${crontab:-<no crontab>}" ;;
    esac
```

The cron repo path becomes a container path: set a `BCRONREPO=/var/backups/arcadia/cron-repo` for the config, and read snapshots from `$T/cron-repo` on the host. Update the redis-volume assertion to the unprefixed path: `grep -qx "/data/redis_data/dump.rdb" "$T/ls"`.

#### 3h — verify the whole migration

- [ ] **Step 13: Shellcheck the changed scripts**

Run: `shellcheck backup/backup.sh backup/lib.sh backup/cron.sh backup/restore.sh scripts/config_value.sh`
Expected: no new warnings (keep the existing `disable=SC2046` directives).

- [ ] **Step 14: Run the docker end-to-end test**

Run: `backup/test.sh docker`
Expected: `backup test passed (docker)`. If `postgres:18-alpine` or `mariadb-dump` auth fails, fix before proceeding (see Review Focus).

- [ ] **Step 15: Confirm the socket is gone**

```bash
docker compose --profile backup config | grep -c docker.sock
```
Expected: `0`.

- [ ] **Step 16: Re-run the host test (must still pass — it shares backup.sh/lib.sh)**

Run: `backup/test.sh host`
Expected: `backup test passed (host)`

- [ ] **Step 17: Commit**

```bash
git add compose.yml backup/cron.Dockerfile backup/cron.sh backup/backup.sh backup/lib.sh backup/test.sh config.example.yml .env.example
git commit -m "feat(backup): docker mode dumps over the network without the docker socket"
```

---

### Task 4: Documentation

**Files:**
- Modify: `docs/src/backup.md`

**Interfaces:** none (prose).

- [ ] **Step 1: Update `backup.md`**

Rewrite the docker-mode sections to reflect: no docker socket and no docker CLI in the backup image; `backup_cron` joins the compose network and dumps `db`/`*_database` by name; restic runs in the container; the `${BACKUP_DIR}` / `RESTIC_PASSWORD_FILE` `.env` settings and the "local-path targets live under `/var/backups/arcadia`" rule; manual run is `docker compose --profile backup run --rm --entrypoint /arcadia/backup/backup.sh backup_cron`; `remote_repo` is now a space-separated list; env vars `<SECTION>_<KEY>` override `config.yml`; `backup.volumes` uses unprefixed names. Add to the limitations section: a volume listed in `backup.volumes` but never populated is backed up empty (the socket-based inspect guard is gone).

- [ ] **Step 2: Verify docs build (if mdbook is available)**

Run: `mdbook build docs` (or skip if mdbook is not installed — this is a prose-only change).
Expected: builds without error, or skipped.

- [ ] **Step 3: Commit**

```bash
git add docs/src/backup.md
git commit -m "docs(backup): document socketless network backup and remote list"
```

---

## Self-review notes

- **Spec coverage:** §Invocation model → Task 3 (3a/3g/3h, `.env.example`, manual command). §1 compose → 3a. §2 Dockerfile → 3b. §3 cron.sh → 3c. §4 backup.sh → 3d. §5 lib.sh → 3e. §6 remote list → Task 2. §7 env layer → Task 1. §8 CI/test.sh → 3g + 3h (test.sh is what `backup.yml` runs; the workflow file itself needs no change — it already triggers on `backup/**` and `compose.yml`). §9 docs → Task 4 (+ config.example in Tasks 1/2/3). §restore unchanged → no task, asserted untouched by 3h Step 16/host run.
- **`.github/workflows/backup.yml`:** no edit needed — it only installs restic + sshd on the host and runs `backup/test.sh docker|host`; both still apply. Confirmed against the file.
- **Type consistency:** `REMOTES` (not `REMOTE`) used in both lib.sh and backup.sh; `dump_mariadb` third arg (`ERGO_DB`/`CHEVERETO_DB`) matches the compose env prefixes and the `eval "${3}_USER"` reads.
