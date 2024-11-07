run:
	cargo build --release
	docker compose up --build -d

build:
	cargo build --release
	docker cp ./target/release/user mubdel-user:/usr/bin/
	docker cp ./target/release/auth mubdel-auth:/usr/bin/
	docker cp ./target/release/payment mubdel-payment:/usr/bin/
	docker restart mubdel-auth mubdel-user mubdel-payment

clean:
	docker kill $(shell docker ps -q)
	docker rm -vf $(shell docker ps -aq)
	docker rmi -f $(shell docker images -aq)
	docker volume rm $(shell docker volume ls -q)
	docker system prune -f
	docker volume prune -f
	docker network prune
