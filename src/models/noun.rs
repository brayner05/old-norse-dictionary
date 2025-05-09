use diesel::{prelude::Queryable, Selectable};
use serde::Serialize;

use crate::schema;


#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = schema::declension_table)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct DeclensionTable {
    pub nom_sg: String,
    pub acc_sg: String,
    pub dat_sg: String,
    pub gen_sg: String,
    pub nom_pl: String,
    pub acc_pl: String,
    pub dat_pl: String,
    pub gen_pl: String
}


#[derive(Queryable, Selectable, Serialize, Clone)]
#[diesel(table_name = schema::noun)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Noun {
    pub word: String,
    pub declension_id: String,
    pub gender: String,
    pub strength: String,
    pub noun_type: i32
}