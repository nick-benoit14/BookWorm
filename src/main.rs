// QUESTION: Is there a good way to create default struct
//   Yes implement the default trait
// https://stackoverflow.com/questions/19650265/is-there-a-faster-shorter-way-to-initialize-variables-in-a-rust-struct

#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate serde;

use rocket_contrib::{Json, Value};

//#[cfg(test)] mod tests; // TODO add tests

pub mod Db {
  extern crate postgres;
  pub use self::postgres::{Connection, TlsMode};
  pub fn get_connection() -> Connection{
   let conn = Connection::connect(
       "postgres://nickbenoit@localhost:5432/book_worm_dev",
       postgres::TlsMode::None).unwrap();
   return conn;
  }
}

pub mod person {
  pub use Db;

  #[derive(Serialize, Deserialize)]
  pub struct Person {
    pub id: Option<i32>,
    pub name: String
  }

  impl Default for Person {
    fn default() -> Person {
      Person { id: None, name: String::new() }
    }
  }

  //TODO figure out how to update id and return to create_dependencies
 pub fn create(person: &Person) -> Person {
   let conn = Db::get_connection();
   let result = conn.execute(
      "INSERT INTO people  (name) VALUES ($1)",
      &[&person.name]
    ).unwrap();
   print!(result);
   Person { ..Default::default() } //TODO
 }
}

pub mod book {
  pub use Db;

  #[derive(Serialize, Deserialize)]
  pub struct Book {
    pub id: Option<i32>,
    pub title: String
  }

  pub fn create(book: &Book) -> Book {
   let conn = Db::get_connection();
   let result = conn.execute(
      "INSERT INTO book  (title) VALUES ($1)",
      &[&book.title]
    ).unwrap();

   //TODO use prepared statemetns
   let stmt = try!(conn.prepare("INSERT INTO foo (bar) VALUES ('baz') RETURNING id"));
   let id: i32 = try!(stmt.query(&[])).iter().next().unwrap().get(0);
   // TODO
   Book { id: Some(0), title: String::from("") }
  }
}

pub mod interaction {
  pub use Db;
  pub use person;
  pub use book;

  #[derive(Serialize, Deserialize)]
  pub struct Interaction {
    id: Option<i32>,
    book: book::Book,
    person: person::Person,
    comment: String,
  }

  //TODO return new interaction
  fn create_dependants<'a>(conn: &'a Db::Connection, interaction: &'a Interaction) -> &'a Interaction {
  if interaction.book.id.is_none() {
    let book = book::create(&interaction.book);
   }

//   if interaction.person.id.is_none() {
//     interaction.person = person::create(&interaction.person);
//   }
   interaction
  }


  pub fn create(interaction: Interaction) -> Interaction{
   let conn = Db::get_connection();
   let result = conn.execute(
      "INSERT INTO interactions  (book_id, person_id, comment) VALUES ($1, $2, $3)",
      &[&interaction.book.id, &interaction.person.id, &interaction.comment]
    ).unwrap();

    interaction
  }

  pub fn list() -> Vec<Interaction> {
    let conn = Db::get_connection();
    let mut result = Vec::new();
    for row in &conn.query(
      "SELECT * FROM interactions i \
         INNER JOIN books on books.id=i.book_id \
         INNER JOIN people on people.id=i.person_id;",
      &[]
     ).unwrap() {
        let interaction  = Interaction {
            id: row.get(0),
            book: book::Book {id: row.get(4), title: row.get(5)},
            person: person::Person {id: row.get(6), name: row.get(7)},
            comment: row.get(3)
        };
        result.push(interaction);
    }

    return result;
  }
}



#[post("/interactions", format = "application/json", data = "<interaction>")]
fn create(interaction: Json<interaction::Interaction>) -> Json<interaction::Interaction>{
  let result = interaction::create(interaction.into_inner());
  Json(result)
}

#[get("/interactions", format = "application/json")]
fn index() -> Json<Vec<interaction::Interaction>>{
  let result = interaction::list();
  Json(result)
}


fn main() {
  rocket::ignite().mount("/", routes![create, index]).launch();
}



/*
extern crate postgres;

use postgres::{Connection, TlsMode};

struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() {
    let conn = Connection::connect("postgres://nickbenoit@localhost:5432/book_worm_dev", // Connect to bookworm_dev
TlsMode::None).unwrap();
    conn.execute("CREATE TABLE person (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR NOT NULL,
                    data            BYTEA
                  )", &[]).unwrap();

    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)",
                 &[&me.name, &me.data]).unwrap();
}

*/
