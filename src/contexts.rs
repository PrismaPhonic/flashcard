use crate::models::Deck;

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexContext {
    pub title: String,
    pub logged_in: bool,
    pub decks: Vec<Deck>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeckContext<'a> {
    pub title: &'static str,
    pub author: &'a str,
    pub logged_in: bool,
}
