use diesel::prelude::*;
use serde::Serialize;
use crate::schema;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = schema::word)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Word {
    pub native_word: String,
    pub english_translation: String,
}