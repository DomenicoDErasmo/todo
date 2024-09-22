-- Your SQL goes here

CREATE TABLE sub_task_map(
    id SERIAL PRIMARY KEY,
    parent_task_id INTEGER NOT NULL,
    CONSTRAINT fk_parent_task
        FOREIGN KEY(parent_task_id)
        REFERENCES task(id),
    sub_task_id INTEGER NOT NULL,
    CONSTRAINT fk_sub_task
        FOREIGN KEY(sub_task_id)
        REFERENCES task(id)

)