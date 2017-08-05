
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

mod person {

  #[derive(Serialize, Deserialize)]
  pub struct Person {
    id: i32,
    name: String
  }
}

// TODO interaction hsould include other types
mod interaction {
  pub use Db;

  #[derive(Serialize, Deserialize)]
  pub struct Interaction {
    book_id: Option<i32>,
    person: String,
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
}



#[post("/interactions", format = "application/json", data = "<interaction>")]
fn create(interaction: Json<interaction::Interaction>) -> Json<interaction::Interaction>{
  let result = interaction::create(interaction.into_inner());
  Json(result)
}


fn main() {
  rocket::ignite().mount("/", routes![create]).launch();
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
    for row in &conn.query("SELECT id, name, data FROM person", &[]).unwrap() {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2),
        };
        println!("Found person {}", person.name);
    }
}

*/
