-- Add migration script here
create table hashmap(key TEXT unique primary key not null, value bytea not null);

create unique index hashmap_idx on hashmap (key);
