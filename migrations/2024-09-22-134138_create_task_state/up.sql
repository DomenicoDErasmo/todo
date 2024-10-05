-- Your SQL goes here

CREATE TABLE task_state (
    id SERIAL PRIMARY KEY,
    description VARCHAR NOT NULL,
    UNIQUE (description)
);

INSERT INTO task_state VALUES (1, 'not_started'), (2, 'in_progress'), (3, 'completed')