#! make
include .env

git_branch = $(shell git rev-parse --abbrev-ref HEAD)
git_time = $(shell git log -1 --format="%at" | xargs -I{} date -d @{} +%FT%T%:z)
build_time = $(shell date +'%FT%T%:z')
database_url = $(shell yq .database.dsn configs/local.yaml)

####
postgres_up:
	docker run --name postgres_test -d \
	-e TZ=Asia/Shanghai -e PGTZ=Asia/Shanghai \
	-e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=secret \
	-p 127.0.0.1:5432:5432 postgres:15-alpine
	# docker-compose -f deployments/postgres.yaml up -d

redis_up:
	docker run --name redis_test -d \
	-e ALLOW_EMPTY_PASSWORD=yes -e REDIS_DISABLE_COMMANDS=FLUSHDB,FLUSHALL \
	-p 127.0.0.1:6379:6379 redis:7-alpine \
	redis-server --save 30 10 --loglevel warning --requirepass world
	# docker-compose -f deployments/redis.yaml up -d

createdb:
	docker exec -it postgres_test createuser hello --createdb --login
	docker exec -it postgres_test psql -c "ALTER ROLE hello PASSWORD 'world'"
	# docker exec -it postgres_test createdb --username=postgres --owner=hello simple_bank
	docker exec -it postgres_test createdb --username=hello --owner=hello simple_bank

connect_db:
	# docker exec -it postgres_test psql --username=postgres simple_bank
	# psql --host=127.0.0.1 --port 5432 --username=hello simple_bank
	docker exec -it postgres_test psql --username=hello simple_bank

connect_cache:
	docker exec -it redis_test redis-cli -a world

dropdb:
	docker exec -it postgres_test dropdb --username=hello simple_bank

migrate_up:
	# migrate -path migrations -database "$(DATABASE_URL)" -verbose up
	migrate -path migrations -database "$(database_url)" -verbose up

migrate_down:
	# migrate -path migrations -database "$(DATABASE_URL)" -verbose down
	migrate -path migrations -database "$(database_url)" -verbose down

.PHONY: postgres_up createdb migrateup

####
build:
	echo ">>> git branch: $(git_branch), git time: $(git_time), build time: $(build_time)"
	mkdir -p target
	go build -o target/main main.go

run:
	mkdir -p target
	go build -o target/main main.go
	./target/main
