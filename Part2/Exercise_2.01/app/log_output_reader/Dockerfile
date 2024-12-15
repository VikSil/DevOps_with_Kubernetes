# Build stage
FROM rust:alpine3.21 as builder

WORKDIR /app

COPY . . 

RUN apk add openssl-dev musl-dev 
RUN RUSTFLAGS="-Ctarget-feature=-crt-static" cargo build --release

# Production stage
FROM rust:alpine3.21

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/log_output_reader .

CMD ["./log_output_reader"]