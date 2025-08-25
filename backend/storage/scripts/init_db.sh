#!/bin/bash

set -e

if [ -z ${DATABASE_URL+x} ]; then
  echo "required envvar DATABASE_URL is not set";
  exit 1;
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