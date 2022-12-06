#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

rustc --version
cargo --version

cargo install cargo-tarpaulin cargo-audit cargo-edit cargo-expand cargo-udeps
rustup component add clippy rustfm

# cargo new zero2prod && cd zero2prod
mkdir -p zero2prod && cd zero2prod
cargo init

mkdir -p tests
cargo add actix-web actix_rt tokio config chrono
cargo add sqlx --features "runtime-actix-rustls macros postgres uuid chrono migrate"
cargo add serde --features=derive
cargo add uuid --features "v4 serde"
cargo add --dev reqwest
cat Cargo.toml

echo 'use_small_heuristics = "Max"' >> .rustfmt.toml

# git add .
# git commit -am "Project skeleton"
# git remote add origin git@github.com:YourGitHubNickName/zero2prod.git
# git push -u origin main

cargo clippy
cargo clippy -- -D warnings

caego fmt
cargo fmt -- --check

cargo audit
cargo expand

cargo run
curl localhost:8000


cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres
command -v sqlx

# postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
export DATABASE_URL=postgres://hello:world@127.0.0.1:5432/newsletter
sqlx database create
# sqlx database drop

psql --host 127.0.0.1 --user hello --port 5432 --password \
  --dbname newsletter -c 'SELECT current_database()'

sqlx migrate add create_subscriptions_table
# edit migrations/{timestamp}_create_subscriptions_table.sql

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

# echo "export DATABASE_URL=$DATABASE_URL" > .env

cargo check

#### docker build sqlx offline mode
# sqlx = { version = "0.6", features = ["runtime-actix-rustls", "macros", "postgres", "uuid", "chrono", "migrate", "offline"] }
cargo install sqlx-cli # install or upgrade
cargo sqlx prepare -- --lib
cat sqlx-data.json

####
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
