# Lets build our rust application in the first stage
FROM alpine:latest as builder

RUN apk add --no-cache curl gcc musl-dev openssl-dev perl make
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ADD . /app

WORKDIR /app

RUN /root/.cargo/bin/cargo build --release
RUN strip target/release/deploy


# The second stage is now really simple
FROM scratch
COPY --from=builder \
    /app/target/release/deploy \
    /usr/local/bin/
WORKDIR /data

ENTRYPOINT ["/usr/local/bin/deploy"]
