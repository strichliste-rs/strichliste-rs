-- Add up migration script here
create table Users (
    id integer not null,
    nickname varchar(255) not null,
    card_number varchar(255),

    primary key (id)
)
