#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate bcrypt;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

use crate::models::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn get_all_decks(conn: &PgConnection) -> Vec<Deck> {
    use self::schema::decks::dsl::*;

    decks.load::<Deck>(conn).expect("Error loading decks")
}

pub fn create_deck<'a>(conn: &PgConnection, tle: &'a str, auth: &'a str) -> Deck {
    use self::schema::decks;

    let new_deck = NewDeck {
        title: tle,
        author: auth,
    };

    diesel::insert_into(decks::table)
        .values(&new_deck)
        .get_result(conn)
        .expect("Error saving new deck")
}

pub fn create_user<'a>(conn: &PgConnection, username: &'a str, password: &'a str) -> User {
    use self::schema::users;

    let hashed_password = bcrypt::hash(password, 10).unwrap();

    let new_user = NewUser {
        username,
        password: &hashed_password
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new deck")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
