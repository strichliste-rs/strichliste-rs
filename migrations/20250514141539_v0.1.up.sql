-- Add up migration script here
create table Users (
    id integer not null,
    nickname varchar(255) not null unique,
    card_number varchar(255),
    money integer not null,

    primary key (id)
)
