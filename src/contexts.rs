use crate::models::Deck;

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexContext {
    pub title: String,
    pub logged_in: bool,
    pub decks: Vec<Deck>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDeckContext<'a> {
    pub title: &'static str,
    pub author: &'a str,
    pub logged_in: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeckContext {
    pub deck: Deck,
    pub jwt: String,
    pub logged_in: bool,
}

