#![feature(proc_macro_hygiene, decl_macro, never_type)]

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate log;

#[macro_use]
extern crate tera;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use] 
extern crate serde;

#[macro_use] 
extern crate serde_derive;

use dotenv::dotenv;
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::http::{Cookie, Cookies};
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FlashMessage, Form, FromRequest, Request};
use rocket::response::status;
use rocket::response::{Flash, Redirect};
use rocket_contrib::databases::diesel;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::path::PathBuf;

use flashcard::contexts::*;
use flashcard::forms::*;
use flashcard::models::{Deck, User};
use flashcard::*;

embed_migrations!("./migrations");

#[database("flashcard_db")]
struct FlashcardDB(diesel::PgConnection);

struct Username(String);

impl<'a, 'r> FromRequest<'a, 'r> for Username {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Username, !> {
        request
            .cookies()
            .get_private("username")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|username| Username(username))
            .or_forward(())
    }
}

#[get("/signup")]
fn signup() -> Template {
    let mut context = HashMap::new();

    context.insert("title", "Sign Up");

    Template::render("login-signup", context)
}

#[get("/login")]
fn login_user(_user: Username) -> Redirect {
    Redirect::to(uri!(index))
}

#[get("/login", rank = 2)]
fn login(flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();

    context.insert("title", "Login");

    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
    }

    Template::render("login-signup", context)
}

#[post("/login", data = "<user>")]
fn handle_login(
    conn: FlashcardDB,
    mut cookies: Cookies,
    user: Form<Signup>,
) -> Result<Redirect, Flash<Redirect>> {
    if validate_password(&conn, &user.username, &user.password) {
        cookies.add_private(Cookie::new("username", user.username.to_string()));
        Ok(Redirect::to(uri!(index)))
    } else {
        Err(Flash::error(
            Redirect::to(uri!(login)),
            "Invalid username/password.",
        ))
    }
}

#[post("/logout")]
fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("username"));
    Flash::success(Redirect::to(uri!(login)), "Successfully logged out.")
}

#[post("/signup", data = "<user>")]
fn handle_signup(
    conn: FlashcardDB,
    mut cookies: Cookies,
    user: Form<Signup>,
) -> Result<Redirect, Flash<Redirect>> {
    let new_user = create_user(&conn, &user.username, &user.password);
    cookies.add_private(Cookie::new("username", new_user.username));
    Ok(Redirect::to(uri!(index)))
}

#[post("/deck", data = "<deck>")]
fn deck(
    conn: FlashcardDB,
    user: Username,
    deck: Form<IncomingDeck>,
) -> Result<Redirect, Flash<Redirect>> {
    create_deck(&conn, &deck.title, &user.0);
    Ok(Redirect::to(uri!(index)))
}

#[get("/")]
fn index(conn: FlashcardDB, _user: Username) -> Template {
    let decks = get_all_decks(&conn);

    let context = IndexContext {
        title: "Home Page".to_string(),
        logged_in: true,
        decks,
    };

    Template::render("index", context)
}

#[get("/", rank = 2)]
fn index_redirect() -> Redirect {
    Redirect::to(uri!(login))
}

#[get("/deck")]
fn deck_form(user: Username) -> Template {
    let context = CreateDeckContext {
        title: "Create New Deck",
        author: &user.0,
        logged_in: true,
    };

    Template::render("create", &context)
}

#[get("/deck/<id>")]
fn deck_details(conn: FlashcardDB, id: i32, user: Username) -> Result<Template, Redirect> {
    if let Ok(deck) = get_one_deck(&conn, id) {
        if deck.author == user.0 {
            let jwt = create_token(user.0);

            let context = DeckContext {
                deck,
                jwt,
                logged_in: true,
            };

            return Ok(Template::render("edit-deck", &context));
        } else {
            return Ok(Template::render("deck-details", &deck));
        }
    }
    Err(Redirect::to(uri!(index)))
}

#[post("/deck/<id>/delete")]
fn handle_delete_deck(conn: FlashcardDB, id: i32, user: Username) -> Result<Redirect, Status> {
    if let Ok(deck) = get_one_deck(&conn, id) {
        if deck.author == user.0 {
            delete_deck(&conn, id).unwrap();
        } else {
            return Err(Status::Unauthorized);
        }
    }

    Ok(Redirect::to(uri!(index)))
}


#[post("/cards", data="<deck>")]
fn handle_add_cards(conn: FlashcardDB, deck: Json<DeckData>) {

    let jwt = &deck.jwt;
    let payload = decode_payload(jwt);

    // verify deck by that id exists - if so unpack it
    if let Ok(_) = get_one_deck(&conn, deck.deck_id) {
        // fix this
        if deck.author == payload.username {
        // loop through and add each card to that deck by FK
            add_cards_to_deck(&conn, deck.deck_id, &deck.cards);
        }
    }
}



/**
 * Error Catchers
 */

// Note - Right now the uri references where we just came from which is invalid
// this works if it was a POST because the link will go to a non-existent GET
// and hit our catch all redirect anyways - but not what we want.
//
// See if there's a way to check two back and redirect there
#[catch(401)]
fn unauthorized(req: &Request) -> Template {
    let mut context = HashMap::new();

    context.insert("error", "You are not authorized!");
    context.insert("logged_in", "true");

    let uri = req.uri().to_string();

    context.insert("direct_msg", "Go back: ");
    context.insert("uri", &uri);
    context.insert("uri_title", "Previous Page");

    Template::render("error", &context)
}

#[get("/<_path..>", rank = 3)]
fn redirect_to_login(_path: PathBuf) -> Redirect {
    Redirect::to(uri!(login))
}

fn rocket() -> rocket::Rocket {
    dotenv().ok();

    rocket::ignite()
        .attach(Template::fairing())
        .attach(FlashcardDB::fairing())
        .attach(AdHoc::on_attach("Run Main Database Migrations", |rocket| {
            let conn = FlashcardDB::get_one(&rocket).expect("database connection");

            match embedded_migrations::run(&*conn) {
                Ok(()) => Ok(rocket),
                Err(e) => {
                    error!("Failed to run database migrations: {:?}", e);
                    Err(rocket)
                }
            }
        }))
        .mount(
            "/",
            routes![
                index,
                index_redirect,
                deck_form,
                redirect_to_login,
                deck,
                handle_delete_deck,
                login,
                logout,
                login_user,
                handle_login,
                signup,
                handle_signup,
                deck_details,
                handle_add_cards,
            ],
        )
        .register(catchers![unauthorized])
}

fn main() {
    rocket().launch();
}
