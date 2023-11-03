#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

docker exec -it --user postgres postgres16_dev bash

pg_ctl status

ls -al /var/lib/postgresql/data/pgdata/

pstree -p postgres

# ps -C postgres -af

ps -af

docker exec -it --user postgres postgres16_dev \
  psql --host 127.0.0.1 --port 5432 --user postgres --dbname postgres

psql -h 127.0.0.1 -p 5432 -U postgres -d postgres

psql -l


user=postgres
db=template1

psql postgresql://$user@localhost:5432/$db

ls $PGDATA/postgresql.conf $PGDATA/pg_hba.conf

cd $PGDARA/base
oid2name

oid2name -d template1 -f 3395

cd pg_tblspc/
oid2name -s

exit
#### postgres commands
# list databases:
\l

# quit:
\q

# editor:
\e

# execute a sql script:
\i test.sql

# help:
\h SELECT

# help with the psql commands
\?

# list tables:
\d

####
select now();
