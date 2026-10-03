# Socketless network backup (docker mode)

## Context

The backup system (commit `d1cab3c7`) runs its docker-mode backups from the
`backup_cron` service, which mounts `/var/run/docker.sock`. That cron container
is always running, so it holds a handle that is effectively root on the host:
it dumps the databases with `docker compose exec`, runs restic from
`restic/restic` via `docker run`, and `cron.sh` starts each scheduled run with a
nested `docker run` — all of which need the socket.

The container does not need any of that. Every service shares the default
compose network, so the backup container can reach `db`, `ergo_database` and
`chevereto_database` by name and dump them over TCP, and it can run restic
in-process against mounted volumes. Dropping the socket removes the largest
privilege the backup container holds and deletes the nested-docker-run
machinery that existed only to work around it.

Intended outcome: docker-mode backups run with no docker socket, no docker CLI,
and no host-published database ports — just the shared compose network.

Scope note: host mode is unchanged. It runs as root on the operator's own box,
where root plus the local unix socket is the appropriate tool. `restore.sh` is
also unchanged — it is run manually on the host by an operator, so the
always-running-container privilege concern does not apply to it; it keeps
`docker compose exec`.

## Invocation model (consequence of going network-based)

Today `backup/backup.sh` runs on the host in docker mode (it only shells out to
`docker compose exec` / `docker run`). Once dumps go over the compose network,
the DB service names (`db`, `ergo_database`, `chevereto_database`) resolve **only
from inside the compose network**, and no DB port is published to the host. So
in docker mode `backup.sh` can no longer run on the host — it must run in a
container joined to the network:

- Scheduled: the long-running `backup_cron` container (via `cron.sh`).
- Manual / one-shot: `docker compose --profile backup run --rm backup_cron
  /arcadia/backup/backup.sh`.

Because restic now runs in that container too, every **local-path** target
(`repo`, `dump_dir`, and a local-path `remote_repo`) must live under the single
`${BACKUP_DIR}` bind mount so it is visible in-container. Backend-URL targets
(`s3:`, `rest:`, `sftp:`) are unaffected. Host mode is unchanged: everything
runs on the host as before.

Restore is still host-run: `restore.sh` reaches the DBs with `docker compose
exec` (not the network), so it keeps working from the host and is unchanged.

## Changes

### 1. `compose.yml` — `backup_cron` service

Remove the `/var/run/docker.sock` mount. Add:

- The three data volumes, read-only, at the `/data/<name>` paths that
  `data_targets()` expects (names keep the `arcadia_` prefix the config lists):
  ```yaml
  - chevereto_storage:/data/arcadia_chevereto_storage:ro
  - ergo_data:/data/arcadia_ergo_data:ro
  - redis_data:/data/arcadia_redis_data:ro
  ```
- One bind mount for the restic repo + dumps, host path via `.env`:
  ```yaml
  - ${BACKUP_DIR:-./backups}:/var/backups/arcadia
  ```
  `backup.repo` and `backup.dump_dir` must both live under `/var/backups/arcadia`
  (the convention that replaces cron.sh's runtime path resolution).
- The password file, read-only, at the configured `password_file` path.
- Mariadb per-database credentials via **YAML anchors** shared with the
  `ergo_database` / `chevereto_database` service `environment:` blocks, so the
  values are defined once. Expose them to `backup_cron` as distinct vars, e.g.
  `ERGO_DB_USER` / `ERGO_DB_PASSWORD` / `ERGO_DB_NAME` and the chevereto
  equivalents.

### 2. `backup/cron.Dockerfile`

Drop `docker-cli` and `docker-cli-compose`. Add `restic`, `postgresql18-client`
(must match `postgres:18.0`) and `mariadb-client` (must match `mariadb:jammy`).

### 3. `backup/cron.sh`

Collapses to: read `backup.cron` + `backup.cron_timezone`, write a crontab that
runs `/arcadia/backup/backup.sh` directly, `exec crond`. Remove the
`docker inspect` label lookups (`PROJECT`, `REPO`, `IMAGE`) and the nested
`docker run` line entirely.

### 4. `backup/backup.sh`

- `dump_postgres`, docker branch → `PGPASSWORD=$(config_value database password)
  pg_dump -h db -U <user> -d <name> --no-owner --no-privileges`. (Postgres creds
  are already in `config.yml`, which is mounted read-only — no env needed.)
- `dump_mariadb`, docker branch → dump the single database as its owning user
  over the network:
  `mariadb-dump -h <service> -u <user> -p<pass> --single-transaction <db>`,
  reading user/pass/db from the env vars passed in step 1. The
  "skip when not running" guard (`docker compose ps`) is replaced by a
  connection attempt that skips the dump if the service is unreachable (an
  empty dump must never be written — it would wipe the DB on restore).
- Host branches of both functions are unchanged.

### 5. `backup/lib.sh`

- `restic()` loses its entire docker branch. restic now always runs locally
  (in the container for docker mode, on the host for host mode), so the function
  is just the env-prefixed `command restic …` that the host branch already is.
- Remove `RESTIC_IMAGE`.
- Remove the docker-socket / restic-image validation. Keep `VOLUMES`, `PATHS`,
  `data_targets()` (volumes still drive both the compose mounts and the snapshot
  targets).

### 6. Remote push targets → list

`backup.remote_repo` becomes space-separated (zero or more). `push()` loops over
each target: init-if-absent, `restic copy --from-repo`, then `forget --prune`.
Validation in `lib.sh` applies the existing per-value check (absolute path or
backend URL) to each entry.

### 7. Env-override layer — `scripts/config_value.sh`

At the top of `config_value()`, check an env var derived from
`<SECTION>_<KEY>` upper-cased (e.g. `BACKUP_REPO` overrides `backup.repo`); if
set and non-empty, print it and return before reading the YAML. Thin shim, no
new config path to parse.

### 8. `backup/test.sh` + `.github/workflows/backup.yml`

Update the docker-mode path to the network approach: no socket, backup runs
restic + the db clients in-container, data volumes mounted. Both modes still
exercised end-to-end, including the sftp pull.

### 9. Docs

`docs/src/backup.md` and `config.example.yml` `backup:` section: document the
`.env` `BACKUP_DIR`, the repo/dumps-under-one-parent convention, the
space-separated `remote_repo`, the env-override layer, and that docker mode no
longer needs the socket or exposed DB ports.

## Out of scope

- `restore.sh` (unchanged).
- Host-mode mariadb dumps (stay root over local socket).
- Replacing the dump logic with an external tool (`nfrastack/db-backup`) —
  rejected: adds a dependency for ~40 working lines.
- Splitting into separate `host-backup.sh` / `docker-backup.sh` — rejected: the
  restic/prune/push logic is mode-independent; once docker mode is network-based
  the branch nearly vanishes, so a split would duplicate code instead of
  removing it.

## Verification

- `backup/test.sh` green for both `docker` and `host` modes (the CI harness
  drives a real compose stack, runs a backup, a restore, and an sftp pull).
- Manual: `docker compose --profile backup run --rm backup_cron
  /arcadia/backup/backup.sh`; confirm `backup_cron` has no socket mount
  (`docker inspect`), and a snapshot lands (restic, against `${BACKUP_DIR}/repo`
  on the host) holding the three `.sql` dumps, the staged config, and the three
  data volumes.
- Confirm `db`/`*_database` still have no `ports:` entry — dumps go over the
  compose network, not host ports.
- Set `remote_repo` to two local paths; confirm both receive the copy.
- Set `BACKUP_REPO` env; confirm it overrides `config.yml`.
