// This file must be kept up to date with the init.sql and models.rs files.

diesel::table! {
    localization (name) {
        name -> Text,
    }
}

diesel::table! {
    content_type (name) {
        name -> Text,
    }
}

diesel::table! {
    directory (id) {
        id -> Integer,
        full_path -> Text,
        relative_path -> Text,
        dir_name -> Text,
        content_type -> Text,
    }
}

diesel::joinable!(directory -> content_type (content_type));

diesel::table! {
    file (id) {
        id -> Integer,
        full_path -> Text,
        relative_path -> Text,
        file_name -> Text,
        content_type -> Text,
    }
}

diesel::joinable!(file -> content_type (content_type));

diesel::table! {
    localization_key (key) {
        key -> Text,
        value -> Text,
        file_id -> Integer,
        localization -> Text,
    }
}

diesel::joinable!(localization_key -> file (file_id));
diesel::joinable!(localization_key -> localization (localization));

diesel::allow_tables_to_appear_in_same_query!(
    localization,
    content_type,
    directory,
    file,
    localization_key
);
