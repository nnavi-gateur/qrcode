#!/bin/sh
set -e

# Ensure the db directory exists and is writable by the unprivileged user.
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

# Generate config.toml on every start so env vars are always reflected.
cat > /run_dir/config.toml << EOF
config_version = 3

[general]
listening_address = "0.0.0.0:8080"
database_path = "./db/db.sqlite"
instance_hostname = "${RS_INSTANCE_HOSTNAME:-https://s.rezel.net}"
hoster_name = "Rezel"
hoster_hostname = "rezel.net"
hoster_tos = "https://rezel.net"
contact = "mailto:rezel@rezel.net"
theme = "light"
cookie_key = "${RS_COOKIE_KEY}"
captcha_difficulty = 3

[phishing]
verbose_console = false
verbose_suspicious = true
verbose_level = "notice"
suspicious_click_count = 25
suspicious_click_timeframe = 12
phishing_password = "${RS_PHISHING_PASSWORD}"
EOF

# Drop privileges and exec the application.
exec gosu unprivileged "$@"
