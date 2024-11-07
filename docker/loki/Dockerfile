FROM debian:latest

RUN apt update && apt install -y wget

RUN wget https://github.com/grafana/loki/releases/download/v2.9.8/loki_2.9.8_amd64.deb

RUN apt install ./loki_2.9.8_amd64.deb

WORKDIR /root/loki

COPY ./docker/loki/config.yaml /root/loki

CMD loki -config.file=/root/loki/config.yaml
