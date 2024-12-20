# Build stage
FROM rust:alpine3.21 as builder

WORKDIR /app

COPY . . 

RUN apk add openssl-dev musl-dev 
RUN RUSTFLAGS="-Ctarget-feature=-crt-static" cargo build --release

# Production stage
FROM rust:alpine3.21

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/todo_backend .

RUN mkdir /usr/local/files && chmod -R 664 /usr/local/files && \ 
    chown -R root:root /usr/local/files && chmod g+s /usr/local/files

CMD ["./todo_backend"]