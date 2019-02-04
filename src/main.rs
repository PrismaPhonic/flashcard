#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate tera;

#[macro_use] extern crate rocket;

use std::collections::HashMap;
use rocket_contrib::templates::Template;

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("Index", "Homepage of the website");
    Template::render("index", &context)
}

fn main() {
    rocket::ignite().attach(Template::fairing()).mount("/", routes![index]).launch();
}
