use super::schema::*;

#[derive(Queryable)]
pub struct Card {
    pub id: i32,
    pub question: String,
    pub answer: String,
    pub deck_id: i32,
    pub user_id: i32,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Queryable)]
pub struct Deck {
    pub id: i32,
    pub title: String,
}

#[derive(Insertable)]
#[table_name="cards"]
pub struct NewCard<'a> {
    pub question: &'a str,
    pub answer: &'a str,
    pub deck_id: i32,
    pub user_id: i32,
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
}
