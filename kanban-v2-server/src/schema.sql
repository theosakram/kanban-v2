-- User
DROP TABLE IF EXISTS tasks;

DROP TABLE IF EXISTS task_columns;

DROP TABLE IF EXISTS boards;

DROP TABLE IF EXISTS users;

CREATE TABLE
  users (
    id serial PRIMARY KEY,
    fullname TEXT NOT NULL,
    username TEXT,
    email TEXT NOT NULL,
    password TEXT NOT NULL
  );

INSERT INTO
  users (fullname, email, password)
VALUES
  (
    'First User',
    'something@gmail.com',
    'somethingsomething'
  );

-- Board
CREATE TABLE
  boards (
    id serial PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users (id)
  );

INSERT INTO
  boards (name, user_id)
VALUES
  ('First Board', 1);

CREATE TABLE
  task_columns (
    id serial PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    board_id INTEGER NOT NULL REFERENCES boards (id)
  );

INSERT INTO
  task_columns (name, board_id)
VALUES
  ('Idle', 1);

INSERT INTO
  task_columns (name, board_id)
VALUES
  ('In Progress', 1);

CREATE TABLE
  tasks (
    id serial PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    serial TEXT,
    column_id INTEGER REFERENCES task_columns (id)
  );

INSERT INTO
  tasks (title, description, column_id)
VALUES
  (
    'First Task',
    'Changing font size on payment page',
    1
  );

INSERT INTO
  tasks (title, description, column_id)
VALUES
  (
    'Second Task',
    'Changing font weight on payment accordion',
    2
  );