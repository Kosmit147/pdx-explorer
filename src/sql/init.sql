PRAGMA foreign_keys = OFF;

DROP TABLE IF EXISTS content_type;
DROP TABLE IF EXISTS directory;
DROP TABLE IF EXISTS file;
DROP TABLE IF EXISTS localization_key;

PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS content_type
(
    type TEXT PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS directory
(
    id           INTEGER PRIMARY KEY,
    path         TEXT NOT NULL,
    content_type TEXT,
    FOREIGN KEY (content_type) REFERENCES content_type (type)
);

CREATE TABLE IF NOT EXISTS file
(
    id           INTEGER PRIMARY KEY,
    path         TEXT NOT NULL,
    content_type TEXT,
    FOREIGN KEY (content_type) REFERENCES content_type (type)
);

CREATE TABLE IF NOT EXISTS localization_key
(
    key     TEXT PRIMARY KEY,
    value   TEXT NOT NULL,
    file_id INTEGER,
    FOREIGN KEY (file_id) REFERENCES file (id)
)