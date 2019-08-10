-- Your SQL goes here
CREATE TABLE todos (
    id SERIAL PRIMARY KEY,
    todo_text VARCHAR NOT NULL,
    time_added VARCHAR NOT NULL,
    time_finished VARCHAR NOT NULL,
    finished BOOLEAN NOT NULL DEFAULT 'f'
)
