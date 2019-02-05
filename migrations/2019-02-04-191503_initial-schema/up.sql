-- Your SQL goes here
CREATE TABLE users (
  username text NOT NULL PRIMARY KEY,
  password text NOT NULL
);

CREATE TABLE decks (
  id serial PRIMARY KEY,
  title text NOT NULL
);

CREATE TABLE cards (
  id serial PRIMARY KEY,
  question text NOT NULL,
  answer text NOT NULL,
  deck_id int NOT NULL REFERENCES decks(id), 
  author text NOT NULL REFERENCES users(username)
);
