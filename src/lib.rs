//! # Flashcard
//!
//! This crate provides a flashcard web application
//!
//! # Setup
//!
//! Make sure that you have the following in your `.env`:
//!
//! ```
//! ROCKET_DATABASES={flashcard_db={url="postgres://username:password@localhost/flashcard"}}
//! ```
//!
//! The most important part is that you don't change the `flashcard_db` text, as this is how rocket
//! will identify which database to connect to when setting up a pool.  Feel free to point the url
//! at any database managing your flashcards.
//!
//! Migrations will be run on rocket launch

#[macro_use]
extern crate diesel;
extern crate bcrypt;
extern crate dotenv;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate rocket;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;
pub mod contexts;
pub mod forms;

use crate::models::*;
use crate::contexts::*;

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

pub fn delete_deck(conn: &PgConnection, deck_id: i32) -> Result<usize, diesel::result::Error> {
    use self::schema::decks::dsl::*;

    diesel::delete(decks.filter(id.eq(deck_id))).execute(conn)
}

pub fn delete_user<'a>(conn: &PgConnection, user: &'a str) -> Result<usize, diesel::result::Error> {
    use self::schema::users::dsl::*;

    diesel::delete(users.filter(username.eq(user))).execute(conn)
}

pub fn create_user<'a>(conn: &PgConnection, username: &'a str, password: &'a str) -> User {
    use self::schema::users;

    let hashed_password = bcrypt::hash(password, 10).unwrap();

    let new_user = NewUser {
        username,
        password: &hashed_password,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new deck")
}

pub fn validate_password<'a>(conn: &PgConnection, u_name: &'a str, pass: &str) -> bool {
    use self::schema::users::dsl::*;

    let user: User = users
        .filter(username.eq(u_name))
        .first(conn)
        .expect("Could not find that user");

    bcrypt::verify(pass, &user.password).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_delete_deck() {
        use super::schema::decks::dsl::*;

        let conn = establish_connection();
        
        let user1 = create_user(&conn, "hackerman", "hackerman");
        let deck1 = create_deck(&conn, "Test title", "hackerman");
        assert_eq!(2 + 2, 4);

        delete_deck(&conn, deck1.id);
        delete_user(&conn, &user1.username);
    }
}
