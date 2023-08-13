FROM rust:1.71.1-alpine3.18 as build

VOLUME .

WORKDIR /opt/src

RUN --mount=target=/opt/src,type=bind,source=. CARGO_TARGET_DIR=/opt/target cargo build --target=aarch64-unknown-linux-musl -p feder_builder

FROM alpine:3.18.3 as run

RUN apk update && \
    apk add shadow && \
    adduser --disabled-password --gecos "" --home /home/feder feder && \
    usermod -p '*' feder && \
    sed -i s/feder:!/"feder:*"/g /etc/shadow

COPY --chown=feder:feder --from=build /opt/target/aarch64-unknown-linux-musl/debug/feder_builder /home/feder/

EXPOSE 8081

USER feder

CMD "/home/feder/feder_builder"
