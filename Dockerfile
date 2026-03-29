FROM rust:latest
WORKDIR /app

COPY . .

RUN cargo build --release

CMD [ "src/main.rs" ]