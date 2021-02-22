
CREATE TABLE tags (
    id          serial          PRIMARY KEY,
    name        text            NOT NULL UNIQUE CHECK(length(name) > 0),
    created_at  timestamp       NOT NULL DEFAULT current_timestamp
);
