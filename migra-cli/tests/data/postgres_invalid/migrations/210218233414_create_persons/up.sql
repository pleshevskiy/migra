-- Your SQL goes here

CREATE TABLE persons (
    id              SERIAL      PRIMARY KEY
    email           text        NOT NULL UNIQUE
    display_name    text        NOT NULL
    created_at      timestamp   NOT NULL DEFAULT current_timestamp
);

ALTER TABLE articles
    ADD COLUMN author_person_id int NULL
        REFERENCES persons (id) ON UPDATE CASCADE ON DELETE CASCADE;
