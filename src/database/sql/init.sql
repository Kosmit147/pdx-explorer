-- This file must be kept up to date with the schema.rs and models.rs files.

PRAGMA foreign_keys = OFF;

DROP TABLE IF EXISTS language;
DROP TABLE IF EXISTS content_type;
DROP TABLE IF EXISTS directory;
DROP TABLE IF EXISTS file;
DROP TABLE IF EXISTS localization_key;

PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS language
(
    name TEXT PRIMARY KEY NOT NULL
);

CREATE TABLE IF NOT EXISTS content_type
(
    name TEXT PRIMARY KEY NOT NULL
);

CREATE TABLE IF NOT EXISTS directory
(
    id            INTEGER PRIMARY KEY NOT NULL,
    full_path     TEXT                NOT NULL,
    relative_path TEXT                NOT NULL,
    dir_name      TEXT                NOT NULL,
    content_type  TEXT,
    FOREIGN KEY (content_type) REFERENCES content_type (name) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS file
(
    id            INTEGER PRIMARY KEY NOT NULL,
    full_path     TEXT                NOT NULL,
    relative_path TEXT                NOT NULL,
    file_name     TEXT                NOT NULL,
    content_type  TEXT,
    FOREIGN KEY (content_type) REFERENCES content_type (name) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS localization_key
(
    key      TEXT PRIMARY KEY NOT NULL,
    value    TEXT             NOT NULL,
    file_id  INTEGER,
    language TEXT,
    FOREIGN KEY (file_id) REFERENCES file (id) ON DELETE CASCADE,
    FOREIGN KEY (language) REFERENCES language (name) ON DELETE CASCADE
)