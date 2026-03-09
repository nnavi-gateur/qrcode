FROM rust:1-bookworm AS builder

WORKDIR /run_dir

COPY . .

RUN cargo build --release --no-default-features --features postgres

FROM debian:bookworm-slim

RUN apt update \
    && apt -y install libpq5 \
    && apt clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /run_dir

# config.toml will be missing and needs to be mounted
COPY --from=builder /run_dir/target/release/rs-short /run_dir/lists.toml /run_dir/lang.json ./

COPY --from=builder /run_dir/assets ./assets

# Ensure /run_dir has proper permissions for the app to write databases
RUN chmod 755 /run_dir

RUN adduser --disabled-password --gecos "" --no-create-home "unprivileged"

# Allow unprivileged user to write to /run_dir for database access
RUN chown -R unprivileged:unprivileged /run_dir

USER unprivileged

CMD ["/run_dir/rs-short"]
