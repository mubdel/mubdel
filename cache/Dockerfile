FROM debian:latest

RUN apt update -y
RUN apt install -y curl

RUN curl -sSf https://install.surrealdb.com | bash

COPY ./migrate /root/migrate
COPY ./import-start.sh /root
RUN chmod +x /root/import-start.sh

EXPOSE 8100

CMD /root/import-start.sh
