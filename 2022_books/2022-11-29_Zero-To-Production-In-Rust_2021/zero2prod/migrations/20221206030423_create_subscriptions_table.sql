-- Add migration script here
CREATE TABLE subscriptions (
  id      uuid NOT NULL,
  PRIMARY KEY (id),
  name    VARCHAR(32)  NOT NULL,
  email   VARCHAR(128) NOT NULL UNIQUE,
  subscribed_at timestamptz NOT NULL
);
