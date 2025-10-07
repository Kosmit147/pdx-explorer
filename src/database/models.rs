// This file must be kept up to date with the init.sql and schema.rs files.

use diesel::prelude::*;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = super::schema::content_type)]
#[diesel(primary_key(name))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ContentType {
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = super::schema::content_type)]
pub struct NewContentType<'a> {
    pub name: &'a str,
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(table_name = super::schema::directory)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(ContentType, foreign_key = content_type))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Directory {
    pub id: i32,
    pub path: String,
    pub content_type: String,
}

#[derive(Insertable)]
#[diesel(table_name = super::schema::directory)]
pub struct NewDirectory<'a> {
    pub id: i32,
    pub path: &'a str,
    pub content_type: &'a str,
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(table_name = super::schema::file)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(ContentType, foreign_key = content_type))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct File {
    pub id: i32,
    pub path: String,
    pub content_type: String,
}

#[derive(Insertable)]
#[diesel(table_name = super::schema::file)]
pub struct NewFile<'a> {
    pub id: i32,
    pub path: &'a str,
    pub content_type: &'a str,
}
