-- database
create database my_db; -- template1;
create database my_db template my_template_db;

update pg_database set datistemplate = true where datname = 'mydb';

-- schema
CREATE SCHEMA customer1;
CREATE SCHEMA customer2;

-- postgresql.conf
-- search_path = "$user", public;

CREATE SCHEMA my_extensions;

ALTER DATABASE mydb SET search_path = '$user', public, my_extensions;

-- role
CREATE ROLE mydb_admin LOGIN PASSWORD 'something';
CREATE DATABASE mydb WITH owner = mydb_admin;

GRANT some_privilege TO some_role;

GRANT ALL ON ALL TABLES IN SCHEMA public TO mydb_admin WITH GRANT OPTION;

GRANT SELECT, REFERENCES, TRIGGER ON ALL TABLES IN SCHEMA my_schema TO PUBLIC;

REVOKE EXECUTE ON ALL FUNCTIONS IN SCHEMA my_schema FROM PUBLIC;


GRANT USAGE ON SCHEMA my_schema TO PUBLIC;

ALTER DEFAULT PRIVILEGES IN SCHEMA my_schema GRANT SELECT, REFERENCES ON TABLES TO PUBLIC;

ALTER DEFAULT PRIVILEGES IN SCHEMA my_schema GRANT ALL ON TABLES TO mydb_admin WITH GRANT OPTION;

ALTER DEFAULT PRIVILEGES IN SCHEMA my_schema GRANT SELECT, UPDATE ON SEQUENCES TO public;

ALTER DEFAULT PRIVILEGES IN SCHEMA my_schema GRANT ALL ON FUNCTIONS TO mydb_admin WITH GRANT OPTION;

ALTER DEFAULT PRIVILEGES IN SCHEMA my_schema GRANT USAGE ON TYPES TO PUBLIC;
