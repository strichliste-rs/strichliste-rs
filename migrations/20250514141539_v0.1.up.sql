-- Add up migration script here
create table Users (
  id integer not null,
  nickname varchar(255) not null unique,
  money integer not null,
  is_system_user boolean not null default false,
  created_at date not null,
  disabled boolean not null default false,
  primary key (id)
);

create table UserCardNumberMap (
  user_id integer not null,
  card_number varchar(255) not null unique,
  primary key (user_id),
  foreign key (user_id) references Users (id)
);

create table Transactions (
  id integer not null,
  sender not null,
  receiver not null,
  is_undone boolean not null default false,
  t_type_data integer,
  money integer unsigned not null,
  description varchar(255),
  timestamp date not null,
  primary key (id),
  foreign key (sender) references Groups (id),
  foreign key (receiver) references Groups (id)
);

create table Articles (
  id integer not null,
  name text not null unique,
  is_disabled boolean not null default false,
  primary key (id)
);

create table ArticleBarcodes (
  article_id integer not null,
  barcode_content text not null unique,
  primary key (barcode_content),
  foreign key (article_id) references Articles (id)
);

create table ArticleCostMap (
  article_id integer not null,
  cost integer not null,
  effective_since date not null,
  foreign key (article_id) references Articles (id)
);

create table Groups (id integer not null, primary key (id));

create table UserGroupMap (
  gid integer not null,
  uid integer not null,
  unique (gid, uid),
  foreign key (gid) references Groups (id),
  foreign key (uid) references Users (id)
);

insert into
  Users (
    id,
    nickname,
    money,
    is_system_user,
    created_at,
    disabled
  )
values
  (0, "snackbar", 0, 1, "1970-01-01 00:00:00", 0),
  (1, "aufladung", 0, 1, "1970-01-01 00:00:00", 0);

insert into
  Groups (id)
values
  (0),
  (1);

insert into
  UserGroupMap (gid, uid)
values
  (0, 0),
  (1, 1);
