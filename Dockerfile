# Build stage
FROM rust:1.82-slim AS builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y 

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/rust-demo .

EXPOSE 8080

CMD ["./rust-demo"]
