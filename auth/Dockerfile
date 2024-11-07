FROM debian:latest

RUN apt update
RUN apt install -y ca-certificates
RUN apt install -y libssl-dev libssl3

COPY ./target/release/auth /usr/bin/

RUN mkdir /etc/mubdel

COPY ./cfg/config/auth-cfg.toml /etc/mubdel

COPY ./errors/messages.ftl /usr/local/src/fluent

RUN chmod +x /usr/bin/auth

EXPOSE 4002

CMD auth -c /etc/mubdel/auth-cfg.toml
