-- A table that denotes each case of a noun.
CREATE TABLE IF NOT EXISTS declension_table (
    declension_table_id     VARCHAR(255) NOT NULL PRIMARY KEY,
    nom_sg                  VARCHAR(128) NOT NULL,
    acc_sg                  VARCHAR(128) NOT NULL,
    dat_sg                  VARCHAR(128) NOT NULL,
    gen_sg                  VARCHAR(128) NOT NULL,
    nom_pl                  VARCHAR(128) NOT NULL,
    acc_pl                  VARCHAR(128) NOT NULL,
    dat_pl                  VARCHAR(128) NOT NULL,
    gen_pl                  VARCHAR(128) NOT NULL
);