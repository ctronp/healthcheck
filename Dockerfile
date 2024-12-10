FROM rust:bookworm AS builder

WORKDIR /usr/src/myapp
COPY Cargo.lock Cargo.toml ./
COPY src ./src
RUN cargo build --release

FROM chainguard/curl:latest-dev

USER root
# DELETE apk
RUN rm -rf $(which apk) $(which ls) $(which which)
USER 65532

WORKDIR /usr/src/myapp
COPY --from=builder /usr/src/myapp/target/release/healthcheck ./

ENTRYPOINT ["./healthcheck"]
