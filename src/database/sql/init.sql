-- This file must be kept up to date with the schema.rs file.

PRAGMA foreign_keys = OFF;

DROP TABLE IF EXISTS content_type;
DROP TABLE IF EXISTS directory;
DROP TABLE IF EXISTS file;
DROP TABLE IF EXISTS localization_key;

PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS content_type
(
    name TEXT PRIMARY KEY NOT NULL
);

CREATE TABLE IF NOT EXISTS directory
(
    id           INTEGER PRIMARY KEY NOT NULL,
    path         TEXT                NOT NULL,
    content_type TEXT,
    FOREIGN KEY (content_type) REFERENCES content_type (name) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS file
(
    id           INTEGER PRIMARY KEY NOT NULL,
    path         TEXT                NOT NULL,
    content_type TEXT,
    FOREIGN KEY (content_type) REFERENCES content_type (name) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS localization_key
(
    key     TEXT PRIMARY KEY NOT NULL,
    value   TEXT             NOT NULL,
    file_id INTEGER,
    FOREIGN KEY (file_id) REFERENCES file (id) ON DELETE CASCADE
)