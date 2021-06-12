-- This file should undo anything in `up.sql`

CREATE TABLE tmp_articles (
    id          int         AUTO_INCREMENT PRIMARY KEY,
    title       text        NOT NULL CHECK (length(title) > 0),
    content     text        NOT NULL,
    created_at  timestamp   NOT NULL DEFAULT current_timestamp
);

INSERT INTO tmp_articles (id, title, content, created_at)
SELECT id, title, content, created_at FROM articles;

DROP TABLE articles;
ALTER TABLE tmp_articles RENAME TO articles;

DROP TABLE persons;
