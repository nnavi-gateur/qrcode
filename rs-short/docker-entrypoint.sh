#!/bin/sh
set -e

# Fix ownership/permissions on the mounted volume at runtime.
# This is necessary because named Docker volumes retain their original
# permissions from when they were first created, overriding Dockerfile
# RUN chown/chmod instructions.
chown -R unprivileged:unprivileged /run_dir
chmod -R u+w /run_dir

# Drop privileges and exec the application.
exec gosu unprivileged "$@"
