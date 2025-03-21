package io.github.brayner05.dictionary.models

import java.util.UUID
import io.github.brayner05.dictionary.util.RuneSpelling

abstract class Word(var word: String, var meanings: Array<String>, var runes: Array<RuneSpelling>) {
    val id: UUID = UUID.randomUUID()
}

