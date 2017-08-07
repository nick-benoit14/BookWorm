create table books(
  id SERIAL,
  title varchar not null,
  primary key(id)
);

create table people(
  id SERIAL,
  name varchar not null,
  primary key(id)
);

create table interactions(
  id SERIAL,
  book_id int not null,
  person_id int not null,
  comment text,
  primary key(id)
);
