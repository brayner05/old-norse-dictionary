package io.github.brayner05.dictionary.models

import io.github.brayner05.dictionary.util.*

class Noun (
    word: String,
    meanings: Array<String>,
    runes: Array<RuneSpelling>,
    var gender: GrammaticalGender,
    var declensions: DeclensionTable,
    var strength: NounStrength,
    var nounType: UByte
) : Word(word, meanings, runes)