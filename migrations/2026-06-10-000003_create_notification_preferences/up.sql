CREATE TABLE user_notification_preferences (
    user_id INTEGER PRIMARY KEY REFERENCES users(id),
    email_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    push_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
