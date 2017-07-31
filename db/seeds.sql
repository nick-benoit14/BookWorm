create table books(
  id int not null,
  title varchar not null,
  primary key(id)
);

create table people(
  id int not null,
  name varchar not null,
  primary key(id)
);

create table interactions(
  id int not null,
  book_id int not null,
  person_id int not null,
  message text,
  primary key(id)
);
