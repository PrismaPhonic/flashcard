
#[derive(FromForm)]
pub struct IncomingDeck {
    pub title: String,
}

#[derive(FromForm)]
pub struct Signup {
    pub username: String,
    pub password: String,
}