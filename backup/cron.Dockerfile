# The backup_cron image: a cron, bash (backup/backup.sh is written in it), the docker cli the job
# drives (docker compose exec, restic from its own image) and tzdata, for backup.cron_timezone.
# The repository is not in it, compose.yml mounts it at /arcadia.
FROM alpine:3.22
RUN apk add --no-cache bash docker-cli docker-cli-compose tzdata
ENTRYPOINT ["/arcadia/backup/cron.sh"]
