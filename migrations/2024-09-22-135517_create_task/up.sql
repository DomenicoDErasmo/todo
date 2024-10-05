-- Your SQL goes here

CREATE TABLE task (
    id SERIAL PRIMARY KEY,
    goal VARCHAR NOT NULL,
    task_state_id INTEGER NOT NULL,
    CONSTRAINT fk_task_state
        FOREIGN KEY(task_state_id)
        REFERENCES task_state(id),
    UNIQUE (goal)
)