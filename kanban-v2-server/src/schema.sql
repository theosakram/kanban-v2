DROP TABLE IF EXISTS tasks;

CREATE TABLE tasks (
  id serial PRIMARY KEY,
  title TEXT NOT NULL,
  description TEXT,
  serial TEXT
);

INSERT INTO tasks(title, description, serial)
VALUES('Deez title', NULL, NULL);

DROP TABLE IF EXISTS users;

CREATE TABLE users (
  id serial PRIMARY KEY,
  fullname TEXT NOT NULL,
  username TEXT,
  email TEXT NOT NULL,
  password TEXT NOT NULL
)

