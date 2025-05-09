use serde::{ser::SerializeStruct, Serialize};

use crate::models::{noun::DeclensionTable, verb::Verb};


#[derive(Serialize)]
pub struct NounDto {
    pub word: String,
    pub declension_table: DeclensionTable,
    pub gender: String,
    pub strength: String,
    pub noun_type: i32
}


pub enum WordDefinition {
    Noun(NounDto),
    Verb(Verb)
}


impl Serialize for WordDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        match self {
            WordDefinition::Noun(noun) => {
                let mut s = serializer.serialize_struct("WordDefinition", 2)?;
                s.serialize_field("type", "Noun")?;
                s.serialize_field("data", noun)?;
                s.end()
            },
            WordDefinition::Verb(verb) => {
                let mut s = serializer.serialize_struct("WordDefinition", 2)?;
                s.serialize_field("type", "Verb")?;
                s.serialize_field("data", verb)?;
                s.end()
            },
        }
    }
}


#[derive(Serialize)]
pub struct WordDto {
    pub native_word: String,
    pub definitions: Vec<WordDefinition>,
}
