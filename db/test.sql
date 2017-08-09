select * from interactions i
  INNER JOIN books on books.id=i.book_id
  INNER JOIN people on people.id=i.person_id;
