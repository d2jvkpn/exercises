create role king login password 'king' valid until 'infinity' createdb;
create role queen login password 'queen' valid until '2023-1-1 00:00' superuser;
-- alter role king with superuser;

-- psql --host 127.0.0.1 --username king --dbname db02 --password

CREATE TABLE person (
	first_name text,
	last_name  text,
	email      text
);

alter table person add constraint name unique(first_name, last_name);

CREATE TABLE place (
	country  text,
	city     text NULL,
	telcode  integer
);

INSERT INTO person (first_name, last_name, email) VALUES
  ('Jason', 'Moiron', 'jmoiron@jmoiron.net'),
  ('John', 'Doe', 'johndoeDNE@gmail.net');

INSERT INTO place (country, city, telcode) VALUES
  ('United States', 'New York', 1),
  ('Hong Kong', '', 852),
  ('Singapore', '', 65);

SELECT * FROM ROWS FROM (
  jsonb_each('{"a":"foo1","b":"bar"}'::jsonb),
  jsonb_each('{"c":"foo2"}'::jsonb))
  x (a1,a1_val,a2_val);
