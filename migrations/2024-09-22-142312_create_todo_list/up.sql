-- Your SQL goes here

CREATE TABLE todo_list (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    owner VARCHAR NOT NULL
)