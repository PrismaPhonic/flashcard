-- This file should undo anything in `up.sql`
ALTER TABLE cards
ADD COLUMN user_id int NOT NULL REFERENCES users(id);

ALTER TABLE decks
DROP COLUMN user_id;
