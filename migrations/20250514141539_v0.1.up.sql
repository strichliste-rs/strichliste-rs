-- Add up migration script here
create table Users (
    id integer not null,
    nickname varchar(255) not null unique,
    money integer not null,

    primary key (id)
);

create table UserCardNumberMap (
    user_id integer not null,
    card_number varchar(255) not null unique,

    primary key (user_id),
    foreign key (user_id)
        references Users(id)
);

create table Transactions (
    id integer not null,
    user_id not null,
    is_undone boolean not null default false,
    t_type text not null,
    t_type_data integer,
    money integer not null,
    description varchar(255),
    timestamp date not null,

    primary key (id),
    foreign key (user_id)
        references Users(id)
);

create table Articles (
    id integer not null,
    name text not null unique,

    primary key (id)
);

create table ArticleBarcodes (
    article_id integer not null,
    barcode_content text not null unique,

    primary key (barcode_content),
    foreign key (article_id)
        references Articles(id)
);

create table ArticleTags (
    id integer not null,
    name text not null,

    primary key (id)
);

create table ArticleTagMap (
    tag_id integer not null,
    article_id integer not null,

    foreign key (tag_id)
        references ArticleTags(id),
    foreign key (article_id)
        references Articles(id)
);

create table ArticleSounds (
    id integer not null,
    name text not null,
    path text not null,

    primary key (id)
);

create table ArticleSoundMap (
    sound_id integer not null,
    article_id integer not null,

      
    foreign key (sound_id)
        references ArticleSounds(id),
    foreign key (article_id)
        references Articles(id)
);

create table ArticleCostMap (
    article_id integer not null,
    cost integer not null,
    effective_since date not null,

    foreign key (article_id)
        references Articles(id)
);
