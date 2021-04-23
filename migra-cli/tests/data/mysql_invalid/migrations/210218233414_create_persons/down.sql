-- This file should undo anything in `up.sql`

ALTER TABLE articles
    DROP COLUMN author_person_id;

DROP TABLE persons;
