package io.github.brayner05.dictionary.controllers

import io.github.brayner05.dictionary.models.Word
import io.github.brayner05.dictionary.repositories.WordRepository
import org.springframework.http.HttpStatus
import org.springframework.http.ResponseEntity
import org.springframework.stereotype.Controller
import org.springframework.ui.Model
import org.springframework.ui.set
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.PathVariable
import java.util.UUID

@Controller
@RequestMapping("/words")
class WordController(val wordRepository: WordRepository) {
    @GetMapping
    fun words(model: Model): String {
        return "word"
    }

    @GetMapping("/{id}")
    fun getWord(@PathVariable id: UUID): ResponseEntity<Word> {
        val matchedWord = wordRepository.getById(id) ?:
            return ResponseEntity
                .status(HttpStatus.NOT_FOUND)
                .body(null)

        return ResponseEntity.ok(matchedWord)
    }
}