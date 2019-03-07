-- Your SQL goes here
ALTER TABLE decks
ADD COLUMN published boolean NOT NULL DEFAULT false;
