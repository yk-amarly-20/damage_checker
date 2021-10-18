-- Your SQL goes here
-- 技データ格納
create table skills
(
  id smallserial primary key,
  name varchar(20) not null,
  type varchar(6) not null,
  dist varchar(1) not null,
  power smallint,
  dai_max_power smallint,
  correction real,
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp
);
