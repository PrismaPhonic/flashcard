#![feature(proc_macro_hygiene, decl_macro, never_type)]

#[macro_use]
extern crate serde;

#[macro_use]
extern crate serde_derive;

use serde::{Deserialize, Serialize};

#[macro_use]
extern crate tera;

#[macro_use]
extern crate rocket;

use rocket::outcome::IntoOutcome;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use rocket::request::{self, Form, FlashMessage, FromRequest, Request};
use rocket::response::{Redirect, Flash};
use std::collections::HashMap;
use rocket::http::{Cookies, Cookie};

use flashcard::models::{ Deck, User };
use flashcard::*;

#[derive(Serialize, Deserialize)]
pub struct IncomingDeck {
    pub title: String,
    pub author: String,
}

#[derive(FromForm)]
pub struct Signup {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct IndexPage {
    title: String,
    logged_in: bool,
    decks: Vec<Deck>,
}

impl IndexPage {
    fn new(title: String, logged_in: bool, decks: Vec<Deck>) -> IndexPage {
        IndexPage { title, logged_in, decks }
    }
}

struct Username(String);

impl<'a, 'r> FromRequest<'a, 'r> for Username {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Username, !> {
        request.cookies()
            .get_private("username")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|username| Username(username))
            .or_forward(())
    }
}

#[get("/")]
fn index(user: Username) -> Template {
    let conn = establish_connection();
    let decks = get_all_decks(&conn);

    let mut context = IndexPage::new("Home Page".to_string(), true, decks);

    Template::render("index", context)
}

#[get("/", rank = 2)]
fn index_redirect() -> Redirect {
    Redirect::to(uri!(login))
}

#[get("/create")]
fn create(user: Username) -> Template {
    let mut context = HashMap::new();

    context.insert("title", "Create New Deck");
    context.insert("author", &user.0);

    Template::render("create", &context)
}

#[get("/create", rank = 2)]
fn redirect_to_login() -> Redirect {
    Redirect::to(uri!(login))
}

#[get("/signup")]
fn signup() -> Template {
    let mut context = HashMap::new();

    context.insert("title", "Sign Up");

    Template::render("login-signup", context)
}

#[get("/login")]
fn login() -> Template {
    let mut context = HashMap::new();

    context.insert("title", "Login");

    Template::render("login-signup", context)
}

#[post("/login", data = "<user>")]
fn handle_login(mut cookies: Cookies, user: Form<Signup>) -> Result<Redirect, Flash<Redirect>> {
    let conn = establish_connection();

    if validate_password(&conn, &user.username, &user.password) {
        cookies.add_private(Cookie::new("username", user.username.to_string()));
        Ok(Redirect::to(uri!(index)))
    } else {
        Err(Flash::error(Redirect::to(uri!(login)), "Invalid username/password."))
    }
}

#[post("/signup", data = "<user>")]
fn handle_signup(mut cookies: Cookies, user: Form<Signup>) -> Result<Redirect, Flash<Redirect>> {
    let conn = establish_connection();

    let new_user = create_user(&conn, &user.username, &user.password);
    cookies.add_private(Cookie::new("username", new_user.username));
    Ok(Redirect::to(uri!(index)))
}

#[post("/deck", data = "<deck>")]
fn deck(deck: Json<IncomingDeck>) {
    let conn = establish_connection();

    create_deck(&conn, &deck.0.title, &deck.0.author);
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index, index_redirect, create, redirect_to_login, deck, login, handle_login, signup, handle_signup])
        .launch();
}
