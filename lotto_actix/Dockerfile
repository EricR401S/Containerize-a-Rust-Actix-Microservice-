FROM rust:latest as builder
ENV APP lotto_actix
WORKDIR /usr/src/$APP
COPY . .
RUN cargo install --path .

# use the testing debian
FROM debian:testing
# FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/$APP /usr/local/bin/$APP
#export this actix web service to port 8080 and 0.0.0.0
EXPOSE 8080
CMD ["lotto_actix"]