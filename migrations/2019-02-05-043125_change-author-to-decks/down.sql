-- This file should undo anything in `up.sql`
ALTER TABLE cards
ADD COLUMN author text NOT NULL REFERENCES users(username);

ALTER TABLE decks
DROP COLUMN author;
