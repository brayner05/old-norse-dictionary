package io.github.brayner05.dictionary.models

data class CaseTable (
    var nominative: String,
    var accusative: String,
    var dative: String,
    var genitive: String
)


data class DeclensionTable (
    var singular: CaseTable,
    var plural: CaseTable,
    var dual: CaseTable?
)