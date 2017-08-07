
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate serde;

use rocket_contrib::{Json, Value};

#[cfg(test)] mod tests;

pub mod Db {
  extern crate postgres;
  use self::postgres::{Connection, TlsMode};
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
    pub id: i32,
    pub name: String
  }
}

pub mod book {
  pub use Db;

  #[derive(Serialize, Deserialize)]
  pub struct Book {
    pub id: i32,
    pub title: String
  }
}

pub mod interaction {
  pub use Db;
  pub use person::{Person};
  pub use book::{Book};

  #[derive(Serialize, Deserialize)]
  pub struct Interaction {
    id: Option<i32>,
    book: Book,
    person: Person,
    comment: String,
  }


  pub fn create(interaction: Interaction) -> Interaction{
   let conn = Db::get_connection();
//   let result = conn.execute(
//      "INSERT INTO interactions  (book_id, person_id, message) VALUES ($1, $2, $3)",
//      &[&interaction.book_id, &me.data]
//    ).unwrap();

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
            book: Book {id: row.get(4), title: row.get(5)},
            person: Person {id: row.get(6), name: row.get(7)},
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
