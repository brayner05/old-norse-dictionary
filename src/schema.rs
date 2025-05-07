// @generated automatically by Diesel CLI.

diesel::table! {
    declension_table (declension_table_id) {
        declension_table_id -> Text,
        nom_sg -> Text,
        acc_sg -> Text,
        dat_sg -> Text,
        gen_sg -> Text,
        nom_pl -> Text,
        acc_pl -> Text,
        dat_pl -> Text,
        gen_pl -> Text,
    }
}

diesel::table! {
    noun (noun_id) {
        noun_id -> Text,
        declension_id -> Text,
        word -> Text,
        gender -> Text,
        strength -> Text,
        noun_type -> Integer,
    }
}

diesel::table! {
    word (word) {
        word -> Text,
        english_translation -> Text,
    }
}

diesel::joinable!(noun -> declension_table (declension_id));
diesel::joinable!(noun -> word (word));

diesel::allow_tables_to_appear_in_same_query!(
    declension_table,
    noun,
    word,
);
