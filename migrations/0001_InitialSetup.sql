CREATE TABLE User (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    hashed_password TEXT NOT NULL,
    email TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE LoginToken (
    user_id INTEGER NOT NULL,
    token TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL,
    valid_for REAL,
    FOREIGN KEY(user_id) REFERENCES User(id)
);

CREATE TABLE Project (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    owner INTEGER NOT NULL,
    FOREIGN KEY(owner) REFERENCES User(id)
);