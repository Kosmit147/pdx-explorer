// This file must be kept up to date with the init.sql file.

use diesel::prelude::*;

table! {
    content_type (name) {
        name -> Text,
    }
}

table! {
    directory (id) {
        id -> Integer,
        path -> Text,
        content_type -> Text,
    }
}

table! {
    file (id) {
        id -> Integer,
        path -> Text,
        content_type -> Text,
    }
}
