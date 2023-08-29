#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

psql --host 127.0.0.1 --username postgres --port 5432 --password

# create user and database
cat <<EOF
create user truuser with password 'mypassword';
create database ezytutors with owner truuser;
-- grant all privileges on database ezytutors to truuser;
EOF

psql --host 127.0.0.1 --username truuser --port 5432 -d ezytutors --password

cat > database.sql <<EOF
/* Drop table if it already exists*/
drop table if exists ezy_course_c5;

/* Create a table. */
/* Note: Don't put a comma after last field */
create table ezy_course_c5  (
  course_id serial primary key,
  tutor_id INT not null,
  course_name varchar(140) not null,
  posted_time TIMESTAMPTZ default now()
);

/* Load seed data for testing */
insert into ezy_course_c5
  (course_id,tutor_id, course_name,posted_time)
  values
  (1, 1, 'First course', '2020-12-17 05:40:00'),
  (2, 1, 'Second course', '2020-12-18 05:45:00');

-- select * from ezy_course_c5;
EOF

psql --host 127.0.0.1 --username postgres --port 5432 --password < database.sql
