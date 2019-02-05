-- Your SQL goes here
ALTER TABLE cards
DROP COLUMN author;

ALTER TABLE decks
ADD COLUMN author text NOT NULL REFERENCES users(username) ON DELETE CASCADE;
