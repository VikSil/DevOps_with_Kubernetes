# Build stage
FROM rust:alpine3.21 as builder

WORKDIR /app

COPY . . 

RUN apk add openssl-dev musl-dev 
RUN RUSTFLAGS="-Ctarget-feature=-crt-static" cargo build --release

# Production stage
FROM rust:alpine3.21

WORKDIR /usr/local/bin

COPY --from=builder /app/migrations ./migrations
COPY --from=builder /app/target/release/todo_backend .

CMD ["./todo_backend"]