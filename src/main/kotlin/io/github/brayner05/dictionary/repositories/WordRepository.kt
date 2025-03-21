package io.github.brayner05.dictionary.repositories
import org.springframework.stereotype.Service
import io.github.brayner05.dictionary.models.CaseTable
import io.github.brayner05.dictionary.models.DeclensionTable
import io.github.brayner05.dictionary.models.Noun
import io.github.brayner05.dictionary.models.Word
import io.github.brayner05.dictionary.util.GrammaticalGender
import io.github.brayner05.dictionary.util.NounStrength
import io.github.brayner05.dictionary.util.RuneSpelling
import java.util.UUID


@Service
class WordRepository : Repository<Word> {
    private val words: Set<Word> = setOf(
        Noun(
            word = "maðr",
            gender = GrammaticalGender.Masculine,
            strength = NounStrength.Strong,
            nounType = 1u,
            declensions = DeclensionTable(
                singular = CaseTable("maðr", "mann", "manni", "mans"),
                plural = CaseTable("menn", "menn", "mǫnnum", "manna"),
                dual = null
            ),
            meanings = arrayOf("man", "person"),
            runes = arrayOf(
                RuneSpelling("ᛘ", "m"),
                RuneSpelling("ᛘᛅᚦᛦ", "maþʀ")
            )
        ),
    )

    override fun getAll(): Array<Word> {
        return words.toTypedArray()
    }

    override fun getById(id: UUID): Word? {
        val word = words.find { it.id == id }
        return word
    }

    override fun add(item: Word) {
        TODO("Not yet implemented")
    }

    override fun updateById(id: UUID, item: Word) {
        TODO("Not yet implemented")
    }

    override fun deleteById(id: UUID) {
        TODO("Not yet implemented")
    }
}