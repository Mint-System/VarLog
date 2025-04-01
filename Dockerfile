FROM rust:slim as builder

WORKDIR /usr/src/varlog

COPY Cargo.toml ./

RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

COPY src ./src

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y libssl-dev ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/varlog/target/release/varlog /usr/local/bin/varlog

WORKDIR /var/log/

EXPOSE 8080

CMD ["varlog"]