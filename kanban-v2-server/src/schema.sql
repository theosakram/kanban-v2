DROP TABLE IF EXISTS tasks;

CREATE TABLE tasks (
  id serial PRIMARY KEY,
  title TEXT NOT NULL,
  description TEXT,
  serial TEXT
);

INSERT INTO tasks(id, title, description, serial)
VALUES(1, 'Deez title', NULL, NULL);