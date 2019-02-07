#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate serde;

#[macro_use]
extern crate serde_derive;

use serde::{Deserialize, Serialize};

#[macro_use]
extern crate tera;

#[macro_use]
extern crate rocket;

use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

use flashcard::models::Deck;
use flashcard::*;

#[derive(Serialize, Deserialize)]
pub struct IncomingDeck {
    pub title: String,
    pub author: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct IndexPage {
    title: String,
    decks: Vec<Deck>,
}

impl IndexPage {
    fn new(title: String, decks: Vec<Deck>) -> IndexPage {
        IndexPage { title, decks }
    }
}

#[get("/")]
fn index() -> Template {
    let conn = establish_connection();
    let decks = get_all_decks(&conn);

    let mut context = IndexPage::new("Home Page".to_string(), decks);

    Template::render("index", context)
}

#[get("/create")]
fn create() -> Template {
    let mut context = HashMap::new();

    context.insert("title", "Create New Deck");

    Template::render("create", context)
}

#[post("/deck", data = "<deck>")]
fn deck(deck: Json<IncomingDeck>) {
    let conn = establish_connection();

    create_deck(&conn, &deck.0.title, &deck.0.author);
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index, create, deck])
        .launch();
}
