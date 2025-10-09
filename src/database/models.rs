// This file must be kept up to date with the init.sql and schema.rs files.

use diesel::prelude::*;

#[derive(Queryable, Identifiable, Selectable, Debug, Clone, PartialEq)]
#[diesel(
    table_name = super::schema::language,
    primary_key(name),
    check_for_backend(diesel::sqlite::Sqlite),
)]
pub struct Language {
    pub name: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = super::schema::language)]
pub struct NewLanguage<'a> {
    pub name: &'a str,
}

#[derive(Queryable, Identifiable, Selectable, Debug, Clone, PartialEq)]
#[diesel(
    table_name = super::schema::content_type,
    primary_key(name),
    check_for_backend(diesel::sqlite::Sqlite),
)]
pub struct ContentType {
    pub name: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = super::schema::content_type)]
pub struct NewContentType<'a> {
    pub name: &'a str,
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, Clone, PartialEq)]
#[diesel(
    table_name = super::schema::directory,
    primary_key(id),
    belongs_to(ContentType, foreign_key = content_type),
    check_for_backend(diesel::sqlite::Sqlite),
)]
pub struct Directory {
    pub id: i32,
    pub full_path: String,
    pub relative_path: String,
    pub dir_name: String,
    pub content_type: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = super::schema::directory)]
pub struct NewDirectory<'a> {
    pub id: i32,
    pub full_path: &'a str,
    pub relative_path: &'a str,
    pub dir_name: &'a str,
    pub content_type: &'a str,
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, Clone, PartialEq)]
#[diesel(
    table_name = super::schema::file,
    primary_key(id),
    belongs_to(ContentType, foreign_key = content_type),
    check_for_backend(diesel::sqlite::Sqlite),
)]
pub struct File {
    pub id: i32,
    pub full_path: String,
    pub relative_path: String,
    pub file_name: String,
    pub content_type: String,
}

#[derive(Queryable, Identifiable, Selectable, Debug, Clone, PartialEq)]
#[diesel(
    table_name = super::schema::file,
    primary_key(id),
    check_for_backend(diesel::sqlite::Sqlite),
)]
pub struct FileIdAndPath {
    pub id: i32,
    pub full_path: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = super::schema::file)]
pub struct NewFile<'a> {
    pub id: i32,
    pub full_path: &'a str,
    pub relative_path: &'a str,
    pub file_name: &'a str,
    pub content_type: &'a str,
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, Clone, PartialEq)]
#[diesel(
    table_name = super::schema::localization_key,
    primary_key(key),
    belongs_to(File, foreign_key = file_id),
    check_for_backend(diesel::sqlite::Sqlite),
)]
pub struct LocalizationKey {
    pub key: String,
    pub value: String,
    pub file_id: i32,
    pub language: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = super::schema::localization_key)]
pub struct NewLocalizationKey<'a> {
    pub key: &'a str,
    pub value: &'a str,
    pub file_id: i32,
    pub language: &'a str,
}
