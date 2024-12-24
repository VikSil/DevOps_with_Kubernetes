# Build stage
FROM rust:slim-buster as builder

WORKDIR /app

COPY . . 

RUN PORT="${PORT:-3055}"
RUN apt-get --yes update && apt-get --yes install pkg-config libssl-dev
RUN cargo install --locked trunk
RUN rustup target add wasm32-unknown-unknown

EXPOSE $PORT

CMD trunk serve --port $PORT