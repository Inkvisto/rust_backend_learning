create table records (id int8 not null, uuid_v4 uuid not null, uuid_v7 uuid not null, filler text);
insert into records select id, gen_random_uuid(), uuid_generate_v7(), repeat(' ', 100) from generate_series(1, 100) id;
