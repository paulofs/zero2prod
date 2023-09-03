-- Create Subscriptions Table
CREATE TABLE subscriptions(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    -- WARN: `timestampz` it's a postress type, need to change to something that works with sqlite3
    subscribed_at timestamptz NOT NULL
);
