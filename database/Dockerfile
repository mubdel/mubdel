FROM debian:latest

RUN apt update -y
RUN apt install -y curl

RUN curl -sSf https://install.surrealdb.com | bash

COPY ./migrate /root/migrate
COPY ./import.sh /root
RUN chmod +x /root/import.sh

EXPOSE 8000

RUN /root/import.sh

CMD surreal start --user root --pass root --bind 0.0.0.0:8000 file:/root/mubdel.db
