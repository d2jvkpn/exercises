CREATE EXTENSION IF NOT EXISTS hstore;

CREATE TABLE products (
    id serial PRIMARY KEY,
    name text,
    properties hstore
);

INSERT INTO products (name, properties)
VALUES ('Product 1', 'color => "red", size => "large"'),
       ('Product 2', 'color => "blue", size => "medium"');

INSERT INTO products (name, properties)
VALUES ('Product 3', 'year => 2023, size => 170');

SELECT properties->'color' FROM products WHERE name = 'Product 1';


UPDATE products SET properties = properties || 'size => "small"' WHERE name = 'Product 2';


select * from products;

SELECT * FROM products WHERE properties = 'color => "red", size => "large"';

SELECT * FROM products WHERE properties = 'color => "red"';

SELECT * FROM products WHERE properties ? 'color';

SELECT * FROM products WHERE properties @> '=> "large"';


CREATE INDEX idx_properties_gist ON products USING gist (properties);

CREATE INDEX idx_properties_gin ON products USING gin (properties);

-- postgresql.conf
-- shared_preload_libraries = 'hstore'
