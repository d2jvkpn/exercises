### Title
---

*https://dev.to/techschoolguru/how-to-write-run-database-migration-in-golang-5h6g*

#### Installation
```bash
go install -tags 'postgres mysql' github.com/golang-migrate/migrate/v4/cmd/migrate@latest

go install github.com/mikefarah/yq/v4@latest
```

#### Create Makefile
...


#### Migration
```bash
migrate create -ext sql -dir migrations -seq init_schema

echo "export DATABASE_URL=postgresql://hello:world@localhost:5433/simple_bank?sslmode=disable" > .env

. .env

migrate -path migrations -database "$DATABASE_URL" -verbose up

migrate -path migrations -database "$DATABASE_URL" -verbose down -all
```
