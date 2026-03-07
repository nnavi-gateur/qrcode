FROM rust:1-bookworm AS builder

WORKDIR /run_dir

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /run_dir

# config.toml will be missing and needs to be mounted
COPY --from=builder /run_dir/target/release/rs-short /run_dir/lists.toml /run_dir/lang.json ./

COPY --from=builder /run_dir/assets ./assets

RUN adduser --disabled-password --gecos "" --no-create-home "unprivileged"

USER unprivileged

CMD ["/run_dir/rs-short"]
