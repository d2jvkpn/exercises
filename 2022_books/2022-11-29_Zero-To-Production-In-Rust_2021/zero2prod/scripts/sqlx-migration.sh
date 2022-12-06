#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

#### install
cargo install --version=0.6.2 sqlx-cli --no-default-features --features native-tls,postgres

command -v sqlx

# postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
export DATABASE_URL=postgres://hello:world@127.0.0.1:5432/newsletter
echo "export DATABASE_URL=$DATABASE_URL" >> .env

sqlx database create

psql --host 127.0.0.1 --user hello --port 5432 --password \
  --dbname newsletter -c 'SELECT current_database()'

# sqlx database drop

#### create table subscriptions
sqlx migrate add create_subscriptions_table
sql_file=$(ls migrations/*_create_subscriptions_table.sql | tail -n 1)

cat >> $sql_file <<EOF
CREATE TABLE subscriptions (
  id      uuid NOT NULL,
  PRIMARY KEY (id),
  name    VARCHAR(32)  NOT NULL,
  email   VARCHAR(128) NOT NULL UNIQUE,
  subscribed_at timestamptz NOT NULL
);
EOF

sqlx migrate run

#### create table subscription_tokens
sqlx migrate add create_subscription_tokens_table

sql_file=$(ls migrations/*_create_subscription_tokens_table.sql | tail -n 1)

cat >> $sql_file <<EOF
CREATE TABLE subscription_tokens(
  subscription_token TEXT NOT NULL,
  subscriber_id      uuid NOT NULL
  REFERENCES         subscriptions (id),
  created_at         timestamptz NOT NULL,
  PRIMARY            KEY (subscription_token)
);
EOF

sqlx migrate run

cargo check

#### 
cargo sqlx prepare -- --lib
cat sqlx-data.json

export SQLX_OFFLINE=true
cargo check
