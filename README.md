# Flashcard

This crate provides a flashcard web application

## Setup

Make sure that you have the following in your `.env`:

```rust
ROCKET_DATABASES={flashcard_db={url="postgres://username:password@localhost/flashcard"}}
```

The most important part is that you don't change the `flashcard_db` text, as this is how rocket
will identify which database to connect to when setting up a pool.  Feel free to point the url
at any database managing your flashcards.

Migrations will be run on rocket launch
