-- Create the table for a 'word'.
-- A word entry contains the original Old Norse word and it's
-- English translation. One word can have multiple meanings.
CREATE TABLE IF NOT EXISTS word (
    native_word             VARCHAR(128) NOT NULL PRIMARY KEY,
    english_translation     VARCHAR(128) NOT NULL
);