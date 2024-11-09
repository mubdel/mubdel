FROM debian:latest

COPY ./target/release/web-server /usr/bin/

COPY ./client/web/dist /var/web

RUN chmod +x /usr/bin/web-server

EXPOSE 4005

CMD web-server
