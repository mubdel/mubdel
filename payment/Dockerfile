FROM debian:latest

RUN apt update
RUN apt install -y ca-certificates
RUN apt install -y libssl-dev libssl3

COPY ./target/release/payment /usr/bin/

RUN mkdir /etc/mubdel

COPY ./cfg/config/payment-cfg.toml /etc/mubdel

COPY ./errors/messages.ftl /usr/local/src/fluent

RUN chmod +x /usr/bin/payment

EXPOSE 4003

CMD payment -c /etc/mubdel/payment-cfg.toml
