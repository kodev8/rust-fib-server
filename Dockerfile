FROM rust:1.75 AS builder
WORKDIR /usr/src/fibonacci-api
COPY . .
RUN cargo install --path .


FROM debian:bookworm-slim
COPY --from=builder /usr/local/cargo/bin/server_devops /usr/local/bin/server_devops

EXPOSE 8000
CMD ["server_devops"]

# docker build -t server_devops .
# docker run -p 8000:8000 server_devops