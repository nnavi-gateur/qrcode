FROM rust:1-bookworm AS builder

WORKDIR /run_dir

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /run_dir

# Install gosu for clean privilege dropping in the entrypoint
RUN apt-get update && apt-get install -y --no-install-recommends gosu \
    && rm -rf /var/lib/apt/lists/*

# config.toml will be missing and needs to be mounted
COPY --from=builder /run_dir/target/release/rs-short /run_dir/lists.toml /run_dir/lang.json ./

COPY --from=builder /run_dir/assets ./assets

COPY docker-entrypoint.sh /docker-entrypoint.sh
RUN chmod +x /docker-entrypoint.sh

RUN adduser --disabled-password --gecos "" --no-create-home "unprivileged"

# Entrypoint fixes volume permissions at runtime then drops to unprivileged
ENTRYPOINT ["/docker-entrypoint.sh"]
CMD ["/run_dir/rs-short"]
