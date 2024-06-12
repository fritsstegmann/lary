CREATE TABLE events(
    id UUID PRIMARY KEY,
    name VARCHAR(255),
    event_type VARCHAR(255),
    data JSON,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
