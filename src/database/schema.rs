// This file must be kept up to date with the init.sql and models.rs files.

use diesel::prelude::*;

table! {
    language (name) {
        name -> Text,
    }
}

table! {
    content_type (name) {
        name -> Text,
    }
}

table! {
    directory (id) {
        id -> Integer,
        full_path -> Text,
        relative_path -> Text,
        dir_name -> Text,
        content_type -> Text,
    }
}

joinable!(directory -> content_type (content_type));
allow_tables_to_appear_in_same_query!(directory, content_type);

table! {
    file (id) {
        id -> Integer,
        full_path -> Text,
        relative_path -> Text,
        file_name -> Text,
        content_type -> Text,
    }
}

joinable!(file -> content_type (content_type));
allow_tables_to_appear_in_same_query!(file, content_type);

table! {
    localization_key (key) {
        key -> Text,
        value -> Text,
        file_id -> Integer,
        language -> Text,
    }
}

joinable!(localization_key -> file (file_id));
allow_tables_to_appear_in_same_query!(localization_key, file);

joinable!(localization_key -> language (language));
allow_tables_to_appear_in_same_query!(localization_key, language);
