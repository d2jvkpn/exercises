postgres_up:
	docker run --name postgres_test -p 127.0.0.1:5433:5432 -e POSTGRES_USER=root -e POSTGRES_PASSWORD=secret -d postgres:15-alpine
	# docker-compose -f deployments/db.yaml up -d

postgres_down:
	docker stop postgres_test
	# docker-compose -f deployments/db.yaml down
	# docker rm postgres_test

createdb:
	docker exec -it postgres_test createuser hello --createdb --login
	docker exec -it postgres_test psql -c "ALTER ROLE hello PASSWORD 'world'"
	# docker exec -it postgres_test createdb --username=root --owner=hello simple_bank
	docker exec -it postgres_test createdb --username=hello --owner=hello simple_bank

connectdb:
	# docker exec -it postgres_test psql --username=root simple_bank
	# psql --host=127.0.0.1 --port 5433 --username=hello simple_bank
	docker exec -it postgres_test psql --username=hello simple_bank

dropdb:
	docker exec -it postgres_test dropdb simple_bank

.PHONY: postgres_up createdb dropdb
