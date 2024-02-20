FROM rust:latest as builder
ENV APP lotto_randomizer
WORKDIR /usr/src/$APP
COPY . .
RUN cargo install --path .

FROM debian:testing-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/$APP /usr/local/bin/$APP
EXPOSE 8080
CMD ["lotto_randomizer"]