-- Add up migration script here
create table Users (
    id integer not null,
    nickname varchar(255) not null unique,
    card_number varchar(255) unique,
    money integer not null,

    primary key (id)
);

create table Transactions (
    id integer not null,
    user_id not null,
    is_undone boolean not null default false,
    t_type text not null,
    origin_user integer,
    destination_user integer,
    money integer not null,
    description varchar(255),
    timestamp date not null,

    primary key (id),
        foreign key (user_id)
            references Users(id),
        foreign key (origin_user)
            references Users(id),
        foreign key (destination_user)
            references Users(id)
);
