-- Your SQL goes here
ALTER TABLE decks 
ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT NOW();