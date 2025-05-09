use core::panic;

use diesel::{
    query_dsl::methods::{
        FilterDsl, 
        LimitDsl, 
        SelectDsl
    }, 
    ExpressionMethods, 
    RunQueryDsl, 
    SelectableHelper
};

use rocket::State;

use crate::{
    database::DbPool, dtos::{
        NounDto, WordDefinition, WordDto
    }, models::{
        noun::{DeclensionTable, Noun}, 
        word::Word
    }, schema::{
        declension_table::{
            dsl::declension_table as declensions_table,
            declension_table_id
        }, 
        noun::{
            self,
            dsl::noun as noun_table,
        }, word::{
            dsl::word as word_table,
            native_word
        }
    }
};


///
/// Query the database for the entire collection of words.
/// This function only retrieves the words however, not the matching
/// noun or verb definitions.
/// 
/// # Params
/// - `pool` - The database connection pool that manages the database connection.
/// 
/// # Returns
/// A `Vec<Word>` containing the list of words
/// 
pub fn get_all_words(pool: &State<DbPool>, limit: Option<i64>) -> Vec<Word> {
    // Retrieve the database connection instance.
    let mut db_connection = pool.get()
        .expect("Failed to access database connection from pool");

    // Query all words.
    let query_result = word_table
        .select(Word::as_select());

    // If a limit was passed, retrieve that many words.
    // Otherwise, retrieve the entire collection of words.
    let words = match limit {
        Some(n) => query_result
            .limit(n)
            .load(&mut db_connection),
            
        None => query_result.load(&mut db_connection)
    };

    words
        .unwrap_or_else(|err| panic!("An error occurred while querying the database: {}", err))
}


///
/// Query the database for each of a word's definitions. A word
/// may have several definitions.
/// 
/// # Params
/// `pool` - The database connection pool that manages the database connection.
/// `word` - The word from which to retrieve the definitions.
/// 
pub fn get_word_definitions(pool: &State<DbPool>, word: &str) -> Option<WordDto> {
    // Retrieve the database connection instance.
    let mut db_connection = pool.get()
        .expect("Failed to access database connection from pool");

    // Query all words.
    let query_result = word_table
        .select(Word::as_select())
        .filter(native_word.eq(word))
        .first(&mut db_connection);

    // If no such word exists, return None
    if query_result.is_err() {
        return None;
    }

    let key = query_result.unwrap().native_word;

    // Get noun definitions of the word
    let noun_definitions = noun_table
        .filter(noun::word.eq(&key))
        .select(Noun::as_select())
        .load(&mut db_connection);

    // Create a WordDto from the information retrieved above
    let dto = WordDto {
        native_word: key.to_string(),
        definitions: noun_definitions.unwrap()
            .iter()
            // Convert each word to a WordDto
            .map(|n| WordDefinition::Noun(
                create_noun_dto(pool, n).unwrap())
            ).collect(),
    };

    Some(dto)
}


///
/// Convert a `Noun` to a `NounDto`.
/// 
fn create_noun_dto(pool: &State<DbPool>, noun: &Noun) -> Option<NounDto> {
    let declension = get_declension_table_by_id(pool, &noun.declension_id);
    if declension.is_none() {
        return None;
    }

    let dto = NounDto { 
        word:  noun.word.to_string(), 
        declension_table: declension.unwrap(), 
        gender: noun.gender.clone(), 
        strength: noun.strength.clone(), 
        noun_type: noun.noun_type 
    };

    Some(dto)
}


///
/// Get the declension table of a noun by it's `declension_table_id`
/// 
/// # Params
/// - `pool` - The database connection pool that manages the database connection.
/// - `id` - The `declension_table_id` of the noun from which to retrieve the declension
/// table.
/// 
fn get_declension_table_by_id(pool: &State<DbPool>, id: &str) -> Option<DeclensionTable> {
    let mut db_connection = pool.get()
        .expect("Failed to access database connection from pool");

    // Get the associated declension table, if it exists
    let declension_table = declensions_table
        .select(DeclensionTable::as_select())
        .filter(declension_table_id.eq(id))
        .first(&mut db_connection);

    match declension_table {
        Ok(dt) => Some(dt),
        Err(_) => None
    }
}