#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate serde;

#[macro_use] extern crate serde_derive;

use serde::{Serialize, Deserialize};

#[macro_use]
extern crate tera;

#[macro_use] extern crate rocket;

use std::collections::HashMap;
use rocket_contrib::templates::Template;

use flashcard::models::Deck;
use flashcard::*;

#[derive(Serialize, Deserialize, Debug)]
struct IndexPage {
    title: String,
    decks: Vec<Deck>,
}

// #[derive(Serialize, Deserialize, Debug)]
// struct Deck {
//     title: String,
//     author: String,
//     id: i32,
// }

// impl Deck {
//     fn new(title: String, author: String, id: i32) -> Deck {
//         Deck {
//             title,
//             author,
//             id,
//         }
//     }
// }

impl IndexPage {
    fn new(title: String, decks: Vec<Deck>) -> IndexPage {
        IndexPage {
            title,
            decks,
        }
    }
}

#[get("/")]
fn index() -> Template {
    // let deck1 = Deck::new("Python Methods".to_string(), "Joel Burton".to_string(), 0);
    // let deck2 = Deck::new("Javascript Methods".to_string(), "Elie Shoppick".to_string(), 1);
    //
    let conn = establish_connection();
    let decks = get_all_decks(&conn);

    let mut context = IndexPage::new("Home Page".to_string(), decks);

    Template::render("index", context)
}

fn main() {
    rocket::ignite().attach(Template::fairing()).mount("/", routes![index]).launch();
}
