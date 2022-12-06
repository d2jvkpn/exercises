-- Add migration script here
CREATE TABLE subscription_tokens(
  subscription_token TEXT NOT NULL,
  subscriber_id      uuid NOT NULL
  REFERENCES         subscriptions (id),
  created_at         timestamptz NOT NULL,
  PRIMARY            KEY (subscription_token)
);
