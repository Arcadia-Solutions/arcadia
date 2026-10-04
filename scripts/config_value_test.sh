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
ARCADIA_BACKUP__REPO=/from/env; export ARCADIA_BACKUP__REPO
[ "$(config_value backup repo)" = /from/env ] || { echo "FAIL: env override"; exit 1; }
ARCADIA_BACKUP__REPO="" ; [ "$(config_value backup repo)" = /from/yaml ] || { echo "FAIL: empty env ignored"; exit 1; }
unset ARCADIA_BACKUP__REPO
# an unrelated key is not shadowed by an unrelated env var
[ -z "$(config_value backup missing)" ] || { echo "FAIL: missing key"; exit 1; }
echo "config_value_test passed"
