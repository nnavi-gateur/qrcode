FROM rust:1-bookworm AS builder

WORKDIR /run_dir

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /run_dir

# Install gosu (privilege dropping) and openssl (secret generation)
RUN apt-get update && apt-get install -y --no-install-recommends gosu openssl \
    && rm -rf /var/lib/apt/lists/*

# config.toml will be missing and needs to be mounted
COPY --from=builder /run_dir/target/release/rs-short /run_dir/lists.toml /run_dir/lang.json ./

COPY --from=builder /run_dir/assets ./assets

COPY docker-entrypoint.sh /docker-entrypoint.sh
RUN chmod +x /docker-entrypoint.sh

RUN adduser --disabled-password --gecos "" --no-create-home "unprivileged"

# Pre-create the db directory with correct ownership
RUN mkdir -p /run_dir/db && chown -R unprivileged:unprivileged /run_dir

# Entrypoint fixes db volume permissions and generates secrets at runtime
ENTRYPOINT ["/docker-entrypoint.sh"]
CMD ["/run_dir/rs-short"]
