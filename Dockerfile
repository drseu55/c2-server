FROM rust:latest

WORKDIR /restapi
COPY . .

RUN cargo install --path .

EXPOSE 8080

CMD ["c2-server"]
