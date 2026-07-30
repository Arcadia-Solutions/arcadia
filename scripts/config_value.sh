# Reads a key of a top level section of the configuration file, e.g. `config_value database user`.
#
# Meant to be sourced, not executed. It exists because the db and redis images only read
# environment variables, and sqlx-cli only reads DATABASE_URL: they cannot be handed the
# configuration file the services read.
#
# The file is the one ARCADIA_CONFIG points at, `config.yml` in the current directory otherwise.
# Only plain scalars are supported, quoted or not. Nothing else of YAML is.
config_value() {
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
