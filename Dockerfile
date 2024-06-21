FROM rust:latest as builder

WORKDIR /usr/src/echoserver-rs
COPY . .
RUN cargo install --path .
RUN cargo build --release
RUN ls -la target/release/

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /usr/src/echoserver-rs/target/release/echoserver-rs /usr/local/bin/echoserver-rs
CMD ["/usr/local/bin/echoserver-rs"]
