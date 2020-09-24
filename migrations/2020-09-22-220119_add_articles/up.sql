-- Your SQL goes here

CREATE TABLE articles (
    id SERIAL NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    link TEXT NOT NULL
)