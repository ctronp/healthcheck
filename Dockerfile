FROM rust:bookworm AS builder

WORKDIR /usr/src/myapp
COPY Cargo.lock Cargo.toml ./
COPY src ./src
RUN cargo build --release

FROM chainguard/curl:latest

WORKDIR /usr/src/myapp
COPY --from=builder /usr/src/myapp/target/release/healthcheck ./

CMD ["./healthcheck"]
