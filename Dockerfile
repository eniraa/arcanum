FROM rust:alpine as builder

WORKDIR /bin/arcanum

COPY Cargo.toml .
COPY src src

RUN cargo build --release

FROM rust:alpine

COPY --from=builder /bin/arcanum/target/release/arcanum /bin/arcanum

ENTRYPOINT ["/bin/arcanum"]
