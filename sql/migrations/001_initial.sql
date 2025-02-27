-- +goose Up
CREATE TABLE users(
                      id INTEGER PRIMARY KEY AUTOINCREMENT ,
                      username TEXT NOT NULL UNIQUE,
                      email TEXT NOT NULL UNIQUE,
                      pass_hash BLOB,
                      salt BLOB,
                      created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

-- +goose Down
DROP TABLE users;