# Reads a key of a top level section of the configuration file, e.g. `config_value database user`.
#
# Meant to be sourced, not executed. It exists because the db and redis images only read
# environment variables, and sqlx-cli only reads DATABASE_URL: they cannot be handed the
# configuration file the services read.
#
# The file is the one ARCADIA_CONFIG points at, `config.yml` in the current directory otherwise.
# Only plain scalars are supported, quoted or not. Nothing else of YAML is.
#
# Environment variables named <SECTION>_<KEY> (upper-cased, '-' -> '_') override the file values.
# Empty or unset env vars fall through to the YAML file.
config_value() {
    # An environment variable named <SECTION>_<KEY> (upper-cased, '-' -> '_') overrides the file,
    # so a deployment can set values without editing config.yml. Empty or unset falls through.
    _cv_env=$(printf '%s_%s' "$1" "$2" | tr '[:lower:]-' '[:upper:]_')
    eval "_cv_val=\${$_cv_env:-}"
    if [ -n "$_cv_val" ]; then printf '%s\n' "$_cv_val"; return; fi
    awk -v section="$1:" -v key="$2:" '
        $1 == section { in_section = 1; next }
        /^[^ ]/ { in_section = 0 }
        in_section && $1 == key {
            value = substr($0, index($0, key) + length(key))
            sub(/^[ \t]+/, "", value)
            sub(/[ \t]+#.*$/, "", value)
            sub(/[ \t]+$/, "", value)
            gsub(/^"|"$|^'"'"'|'"'"'$/, "", value)
            print value
            exit
        }
    ' "${ARCADIA_CONFIG:-config.yml}"
}
