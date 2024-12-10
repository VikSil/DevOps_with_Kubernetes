# Build stage
FROM rust:alpine3.21 as builder

WORKDIR /app

COPY . . 

RUN apk add musl-dev && cargo build --release

# Production stage
FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/log_output .

CMD ["./log_output"]