FROM debian:latest

RUN apt update
RUN apt install -y ca-certificates
RUN apt install -y libssl-dev libssl3

COPY ./target/release/user /usr/bin/

RUN mkdir /etc/mubdel

COPY ./cfg/config/user-cfg.toml /etc/mubdel/

RUN mkdir /usr/local/templates

COPY ./email/templates /usr/local/src/templates

COPY ./errors/messages.ftl /usr/local/src/fluent

RUN chmod +x /usr/bin/user

EXPOSE 4001

CMD user -c /etc/mubdel/user-cfg.toml
