use diesel::prelude::*;
use serde::Serialize;

use crate::schema;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = schema::word)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Word {
    native_word: String,
    english_translation: String
}