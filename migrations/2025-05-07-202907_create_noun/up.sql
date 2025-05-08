CREATE TABLE IF NOT EXISTS noun (
    noun_id         VARCHAR(255) NOT NULL PRIMARY KEY,
    declension_id   VARCHAR(255) NOT NULL,
    word            VARCHAR(128) NOT NULL,
    gender          VARCHAR(32) NOT NULL 
                                CHECK(gender IN ('masculine', 'feminine', 'neuter')),
    strength        VARCHAR(32) NOT NULL
                                CHECK(strength IN ('strong', 'weak')),
    noun_type       INTEGER NOT NULL
                                CHECK(noun_type > 0 AND noun_type <= 4),

    FOREIGN KEY (declension_id) REFERENCES declension_table(declension_table_id),
    FOREIGN KEY (word) REFERENCES word(native_word)
);