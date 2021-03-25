-- Your SQL goes here

CREATE TABLE articles (
    id          serial      PRIMARY KEY,
    title       text        NOT NULL CHECK (length(title) > 0),
    content     text        NOT NULL,
    created_at  timestamp   NOT NULL DEFAULT current_timestamp
);
