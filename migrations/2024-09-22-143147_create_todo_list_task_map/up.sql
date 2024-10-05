-- Your SQL goes here

CREATE TABLE todo_list_task_map (
    id SERIAL PRIMARY KEY,
    todo_list_id INTEGER NOT NULL,
    CONSTRAINT fk_todo_list
        FOREIGN KEY (todo_list_id)
        REFERENCES todo_list(id),
    task_id INTEGER NOT NULL,
    CONSTRAINT fk_task
        FOREIGN KEY (task_id)
        REFERENCES task(id),
    UNIQUE (todo_list_id, task_id)
)