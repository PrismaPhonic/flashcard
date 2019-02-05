-- This file should undo anything in `up.sql`
ALTER TABLE decks
DROP COLUMN created_at;
