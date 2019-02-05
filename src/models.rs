use serde_derive::*;
use super::schema::*;
use chrono::naive::{ NaiveDate, NaiveDateTime };

#[derive(Queryable)]
pub struct Card {
    pub id: i32,
    pub question: String,
    pub answer: String,
    pub deck_id: i32,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Deck {
    pub id: i32,
    pub title: String,
    pub created_at: NaiveDateTime, 
    pub author: String,
}

#[derive(Insertable)]
#[table_name="cards"]
pub struct NewCard<'a> {
    pub question: &'a str,
    pub answer: &'a str,
    pub deck_id: i32,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Insertable)]
#[table_name="decks"]
pub struct NewDeck<'a> {
    pub title: &'a str,
    pub author: &'a str,
}
