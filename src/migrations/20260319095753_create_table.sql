-- Add migration script here
CREATE TABLE IF NOT EXISTS  users (
    user_id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_name TEXT UNIQUE,
    user_uuid TEXT,
    user_password TEXT,
    user_email TEXT
);

CREATE TABLE IF NOT EXISTS session(
    session_id INTEGER PRIMARY KEY AUTOINCREMENT ,
    user_uuid TEXT UNIQUE,
    user_name TEXT UNIQUE,
    session_experation TEXT
);

CREATE TABLE IF NOT EXISTS tasks(
    id INTEGER PRIMARY KEY AUTOINCREMENT ,
    task_name TEXT,
    task_value TEXT,
    user_uuid TEXT UNIQUE,
    user_name TEXT UNIQUE,
    task_data TEXT
);


