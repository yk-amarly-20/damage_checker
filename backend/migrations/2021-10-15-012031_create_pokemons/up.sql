-- Your SQL goes here
-- ポケモンデータ格納
create table pokemons
(
  id smallserial primary key,
  book_number smallint not null,
  name varchar(30) not null,
  en_name varchar(30) not null,
  weight real not null,
  ketaguri smallint not null,
  type1 varchar(6) not null,
  type2 varchar(6),
  status_h smallint not null,
  status_a smallint not null,
  status_b smallint not null,
  status_c smallint not null,
  status_d smallint not null,
  status_s smallint not null,
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp
);
