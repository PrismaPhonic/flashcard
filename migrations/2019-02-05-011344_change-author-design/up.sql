-- Your SQL goes here
ALTER TABLE cards
DROP COLUMN user_id;

ALTER TABLE decks
ADD COLUMN user_id int NOT NULL REFERENCES users(id) ON DELETE CASCADE;
