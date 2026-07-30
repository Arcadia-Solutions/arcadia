#!/bin/bash

set -e

# sqlx-cli only reads DATABASE_URL, so it is built from the `database` section of the
# configuration file when it is not already set.
if [ -z ${DATABASE_URL+x} ]; then
  ARCADIA_CONFIG="${ARCADIA_CONFIG:-/app/config.yml}"

  if [ ! -f "$ARCADIA_CONFIG" ]; then
    echo "neither DATABASE_URL nor the configuration file '$ARCADIA_CONFIG' is available";
    exit 1;
  fi

  # Mounted next to this script by compose.yml
  . /config_value.sh

  export DATABASE_URL="postgresql://$(config_value database user):$(config_value database password)@$(config_value database host):$(config_value database port)/$(config_value database name)"
fi

# Add retry logic
max_attempts=30
attempt=1

while [ $attempt -le $max_attempts ]; do
  echo "Attempt $attempt: Trying to connect to database..."
  if cargo sqlx database setup --source ./migrations; then
    echo "Database setup successful!"
    exit 0
  else
    echo "Database connection failed, retrying in 2 seconds..."
    sleep 2
    attempt=$((attempt + 1))
  fi
done

echo "Failed to connect to database after $max_attempts attempts"
exit 1
