FROM rust:slim as builder

WORKDIR /usr/src/varlog

COPY Cargo.* ./

COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y libssl-dev ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/varlog/target/release/varlog /usr/local/bin/varlog

CMD ["varlog"]