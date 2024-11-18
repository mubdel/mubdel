FROM debian:latest

RUN apt update
RUN apt install -y ca-certificates
RUN apt install -y libssl-dev libssl3

COPY ./target/release/crypto-flow /usr/bin/

RUN mkdir /etc/mubdel

COPY ./cfg/config/crypto-flow-cfg.toml /etc/mubdel/

RUN mkdir /usr/local/templates

COPY ./email/templates /usr/local/src/templates

COPY ./errors/messages.ftl /usr/local/src/fluent

RUN chmod +x /usr/bin/crypto-flow

EXPOSE 4004

CMD crypto-flow -c /etc/mubdel/crypto-flow-cfg.toml
