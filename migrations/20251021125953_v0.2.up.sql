-- Add up migration script here
create table UserPreferences (
  user_id integer not null,
  alternative_coloring boolean not null default false,
  primary key (user_id),
  foreign key (user_id) references Users (id)
)
