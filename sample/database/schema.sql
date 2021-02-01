
create table tags (
    id          serial          primary key,
    name        text            not null unique check(length(name) > 0),
    created_at  timestamp       not null default current_timestamp
);
