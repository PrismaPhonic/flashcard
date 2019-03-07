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
use jsonwebtoken::{encode, decode, Algorithm, Header, Validation};
use chrono::Utc;

pub mod models;
pub mod schema;
pub mod contexts;
pub mod forms;

use crate::models::*;
use crate::contexts::*;

#[derive(Deserialize, Debug)]
pub struct DeckData {
    pub author: String,
    pub deck_id: i32,
    pub cards: Vec<NewCardJSON>,
    pub jwt: String,
}

#[derive(Deserialize, Debug)]
pub struct NewCardJSON {
    pub question: String,
    pub answer: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
   pub username: String,
   pub exp: i64,
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn get_all_decks(conn: &PgConnection) -> Vec<Deck> {
    use self::schema::decks::dsl::*;

    decks.load::<Deck>(conn).expect("Error loading decks")
}

pub fn get_one_deck(conn: &PgConnection, deck_id: i32) -> Result<Deck, diesel::result::Error> {
    use self::schema::decks::dsl::*;

    decks.find(deck_id)
        .first(conn)
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

pub fn add_cards_to_deck(conn: &PgConnection, deck_id: i32, cards: &Vec<NewCardJSON>) {
    for card in cards {
        let NewCardJSON { question, answer } = card;
        create_card(conn, &question, &answer, deck_id);
    }
}


pub fn create_card<'a>(conn: &PgConnection, question: &'a str, answer: &'a str, deck_id: i32) -> Card {
    use self::schema::cards;

    let new_card = NewCard {
        question,
        answer,
        deck_id
    };

    diesel::insert_into(cards::table)
        .values(&new_card)
        .get_result(conn)
        .expect("Error saving new card")
}

pub fn validate_password<'a>(conn: &PgConnection, u_name: &'a str, pass: &str) -> bool {
    use self::schema::users::dsl::*;

    let user: User = users
        .filter(username.eq(u_name))
        .first(conn)
        .expect("Could not find that user");

    bcrypt::verify(pass, &user.password).unwrap()
}

pub fn create_token(username: String) -> String {

    // set timeout on JWT to 30 minutes (1800 seconds) after you get it
    let new_payload = Payload {
        username,
        exp: Utc::now().timestamp() + 1800,
    };

    encode(&Header::default(), &new_payload, "testkey".as_ref()).unwrap()
}

pub fn decode_payload(token: &str) -> Payload {
    let token_data = decode::<Payload>(token, b"testkey", &Validation::default()).unwrap();
    token_data.claims
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_decks_model() {
        use super::schema::decks::dsl::*;

        let conn = establish_connection();
        
        let user1 = create_user(&conn, "hackerman", "hackerman");
        let deck1 = create_deck(&conn, "Test title", "hackerman");

        assert_eq!(Ok(deck1.id), decks.find(deck1.id).select(id).first(&conn));

        delete_deck(&conn, deck1.id);
        delete_user(&conn, &user1.username);

        assert_ne!(Ok(deck1.id), decks.find(deck1.id).select(id).first(&conn));
    }
}
