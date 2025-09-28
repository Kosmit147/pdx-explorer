PRAGMA foreign_keys = ON;

DROP TABLE IF EXISTS file;
DROP TABLE IF EXISTS localization_key;

CREATE TABLE IF NOT EXISTS file
(
    path TEXT PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS localization_key
(
    key       TEXT PRIMARY KEY,
    value     TEXT NOT NULL,
    file_path TEXT,
    FOREIGN KEY (file_path)
        REFERENCES file (path)
)