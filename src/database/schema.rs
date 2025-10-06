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

joinable!(directory -> content_type (content_type));
allow_tables_to_appear_in_same_query!(directory, content_type);

table! {
    file (id) {
        id -> Integer,
        path -> Text,
        content_type -> Text,
    }
}

joinable!(file -> content_type (content_type));
allow_tables_to_appear_in_same_query!(file, content_type);
