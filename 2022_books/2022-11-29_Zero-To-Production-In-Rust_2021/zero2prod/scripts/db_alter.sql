CREATE TYPE public.subscriptions_status AS ENUM('confirmed', 'cancelled', 'deleted');

ALTER TABLE public.subscriptions ADD COLUMN status subscriptions_status DEFAULT 'confirmed'
  AFTER subscribed_at;
