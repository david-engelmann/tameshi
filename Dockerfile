FROM rust:1.85-bookworm AS builder
WORKDIR /src
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --locked --bin tameshi

FROM debian:bookworm-slim
RUN groupadd -g 65532 nonroot && useradd -u 65532 -g 65532 -m -s /usr/sbin/nologin nonroot
COPY --from=builder /src/target/release/tameshi /usr/local/bin/tameshi
USER 65532:65532
WORKDIR /work
ENTRYPOINT ["/usr/local/bin/tameshi"]
