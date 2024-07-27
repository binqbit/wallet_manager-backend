FROM rustlang/rust:nightly-slim AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev

WORKDIR /home/rust/src/backend
COPY . .
RUN cargo install --locked --path .

FROM ubuntu:latest
RUN apt-get update && apt-get install -y libssl-dev ca-certificates
COPY --from=builder /usr/local/cargo/bin/backend .
COPY ./config ./config
EXPOSE 8000

CMD ["./backend"]
