#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate serde;

use rocket_contrib::{Json, Value};

//#[cfg(test)] mod tests; // TODO add tests

pub mod db {
  extern crate postgres;
  pub use self::postgres::{Connection, TlsMode};
  pub fn get_connection() -> Connection{
   let conn = Connection::connect(
       "postgres://nickbenoit@localhost:5432/book_worm_dev",
       postgres::TlsMode::None).unwrap();
   return conn;
  }
}

pub mod token {
  pub use db;

  pub struct Token {
    pub id: Option<i32>,
    pub key: String,
  }
}

pub mod person {
  pub use db;

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

 pub fn create(conn: &db::Connection, person: Person) -> Person {
   let stmt = conn
       .prepare("INSERT INTO people (name) VALUES ($1) RETURNING id")
       .unwrap();
   let result = stmt.query(&[&person.name]).unwrap();
   let id = result.get(0).get(0);
   Person { id: Some(id), name: person.name }
 }
}

pub mod book {
  pub use db;

  #[derive(Serialize, Deserialize)]
  pub struct Book {
    pub id: Option<i32>,
    pub title: String
  }

  impl Default for Book {
    fn default() -> Book {
      Book { id: None, title: String::new() }
    }
  }
  pub fn create(conn: &db::Connection, book: Book) -> Book {
   let stmt = conn
       .prepare("INSERT INTO books (title) VALUES ($1) RETURNING id")
       .unwrap();
   let result = stmt.query(&[&book.title]).unwrap();
   let id = result.get(0).get(0);
   Book { id: Some(id), title: book.title }
  }
}

pub mod interaction {
  use std::fmt;
  pub use db;
  pub use person;
  pub use book;

  #[derive(Serialize, Deserialize)]
  pub struct Interaction {
    id: Option<i32>,
    book: book::Book,
    person: person::Person,
    comment: String,
  }

  impl Default for Interaction {
    fn default() -> Interaction {
      Interaction {
          id: None,
          book: book::Book { ..Default::default() },
          person: person::Person { .. Default::default() },
          comment: String::new(),
      }
    }
  }

  impl fmt::Debug for Interaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(
        f,
        "Interaction {{ id: {:?}, book_id: {:?}, person_id: {:?}, comment: {:?}}}",
        self.id, self.book.id, self.person.id, self.comment
      )
    }
  }


  fn create_dependants<'a>(conn: &'a db::Connection, interaction: Interaction) ->  Result<Interaction, ()> {
   let book = if interaction.book.id.is_none() {
      book::create(conn, interaction.book)
   } else {
      interaction.book
   };

   let person = if interaction.person.id.is_none() {
      person::create(conn, interaction.person)
   } else {
      interaction.person
   };
   Ok(Interaction { book: book, person: person, ..interaction })
  }


  pub fn create(interaction: Interaction) -> Interaction{
   let conn = db::get_connection();

   let stmt = conn
     .prepare(
       "INSERT INTO interactions  (book_id, person_id, comment) VALUES ($1, $2, $3) RETURNING id")
     .unwrap();

   let updated_interaction = create_dependants(&conn, interaction);
   match updated_interaction {
      Ok(v) => {
       let result = stmt.query(&[
         &v.book.id,
         &v.person.id,
         &v.comment
       ]).unwrap();
       let id = result.get(0).get(0);
       Interaction { id: id, ..v }
     }
     Err(e) => {
       println!("error::; {:?}", e);
       Interaction { ..Default::default() }
     }
   }
  }

  pub fn list() -> Vec<Interaction> {
    let conn = db::get_connection();
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

  pub fn find(id: i32) -> Interaction {
    let conn = db::get_connection();
    let stmt = conn
      .prepare(
         "SELECT * FROM interactions i \
         INNER JOIN books on books.id=i.book_id \
         INNER JOIN people on people.id=i.person_id \
         WHERE i.id=$1 LIMIT 1;",
      ).unwrap();
    let result = stmt.query(&[&id]).unwrap();
    let row = result.get(0);
    Interaction {
         id: row.get(0),
         book: book::Book {id: row.get(4), title: row.get(5)},
         person: person::Person {id: row.get(6), name: row.get(7)},
         comment: row.get(3)
    }
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

#[get("/interactions/<id>", format = "application/json")]
fn show(id: i32) -> Json<interaction::Interaction>{
  let result = interaction::find(id);
  Json(result)
}


fn main() {
  rocket::ignite().mount("/", routes![create, index, show]).launch();
}

