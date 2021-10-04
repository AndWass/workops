CREATE TABLE user (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    hashed_password TEXT NOT NULL,
    email TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE login_token (
    user_id INTEGER NOT NULL,
    token TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL,
    valid_for REAL,
    FOREIGN KEY(user_id) REFERENCES user(id)
);

CREATE TABLE project (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    owner INTEGER NOT NULL,

    created_at TEXT NOT NULL,

    FOREIGN KEY(owner) REFERENCES user(id)
);

CREATE TABLE work_item (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    type INTEGER NOT NULL, -- 0 = bug

    project_id INTEGER NOT NULL,
    created_by INTEGER NOT NULL,
    created_at STRING NOT NULL,

    title STRING NOT NULL,
    description STRING NOT NULL,

    assigned_to INTEGER NOT NULL,
    state INTEGER NOT NULL, -- 0 = New
    state_reason INTEGER NOT NULL, -- 0 = New

    FOREIGN KEY(project_id) REFERENCES project(id),
    FOREIGN KEY(created_by) REFERENCES user(id),
    FOREIGN KEY(assigned_to) REFERENCES user(id)
);