run:
	cargo build --release
	grass client/web/assets/sass/main.scss client/web/assets/main.css
	cd client/web && dx build --release && cd -
	docker compose up --build -d

build:
	cargo build --release
	docker cp ./target/release/user mubdel-user:/usr/bin/
	docker cp ./target/release/auth mubdel-auth:/usr/bin/
	docker cp ./target/release/payment mubdel-payment:/usr/bin/
	docker cp ./target/release/crypto-flow mubdel-crypto-flow:/usr/bin/
	docker restart mubdel-auth mubdel-user mubdel-payment mubdel-crypto-flow
	grass client/web/assets/sass/main.scss client/web/assets/main.css
	cd client/web && dx build --release && cd -
	docker cp ./client/web/dist mubdel-web:/var/web
	docker cp ./target/release/web-server mubdel-web:/usr/bin/
	docker restart mubdel-web

clean:
	docker kill $(shell docker ps -q)
	docker rm -vf $(shell docker ps -aq)
	docker rmi -f $(shell docker images -aq)
	docker volume rm $(shell docker volume ls -q)
	docker system prune -f
	docker volume prune -f
	docker network prune
