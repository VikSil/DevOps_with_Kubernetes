# Build stage
FROM rust:alpine3.21 as builder

WORKDIR /app

COPY . . 

RUN apk add musl-dev && cargo build --release

# Production stage
FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/log_output_writer .

RUN mkdir /usr/local/files && chmod -R 664 /usr/local/files && \ 
    chown -R root:root /usr/local/files && chmod g+s /usr/local/files

CMD ["./log_output_writer"]