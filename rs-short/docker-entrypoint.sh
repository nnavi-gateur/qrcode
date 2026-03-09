#!/bin/sh
set -e

# Ensure the db directory exists and is writable by the unprivileged user.
# Named Docker volumes retain permissions from their first creation, so we
# fix them here at every startup.
mkdir -p /run_dir/db
chown unprivileged:unprivileged /run_dir/db
chmod u+rwx /run_dir/db

# Auto-generate secrets if not provided via environment variables.
if [ -z "$RS_COOKIE_KEY" ]; then
    export RS_COOKIE_KEY=$(openssl rand -base64 64 | tr -d '\n')
fi

if [ -z "$RS_PHISHING_PASSWORD" ]; then
    export RS_PHISHING_PASSWORD=$(openssl rand -hex 16)
fi

# Drop privileges and exec the application.
exec gosu unprivileged "$@"
