package sh.adamcooper.wordle

import java.net.URI
import java.net.http.HttpClient
import java.net.http.HttpRequest
import java.net.http.HttpResponse
import java.time.LocalDate
import java.util.logging.Logger

/**
 * Groups together Wordle library functions along with cached state of online resources
 */
class Wordle {
    val wordList: List<String> by lazy {
        Wordle.log.info("Fetching Wordle word list")
        val httpClient = HttpClient.newHttpClient()
        val request = HttpRequest.newBuilder(URI.create(WORD_LIST_URL)).build()
        val response = httpClient.send(request, HttpResponse.BodyHandlers.ofString())
        response.body().splitToSequence("\n").map(String::trim).map(String::uppercase).filter(String::isNotEmpty).toList()
    }
    val bestWord: String by lazy {
        Wordle.log.info("Calculating best word")
        this.findBestWords(
            this.wordList,
            n = 1,
            uniqueOnly = true
        ).asSequence().first().key.also {
            Wordle.log.info("Found best word $it")
        }
    }

    /**
     * Finds the best word in the word list by counting the most common characters and then
     * scoring all the words with unique characters based on those counts.
     */
    @Suppress("MagicNumber")
    fun findBestWords(
        words: List<String> = this.wordList,
        n: Int = 5,
        uniqueOnly: Boolean = false
    ): Map<String, Int> {
        if (words.size == 1) {
            return mapOf(words.first() to 0)
        }
        // Initialize counter of uppercase alphabet mapped to 0
        val counter = ('A'..'Z').associateWith { 0 }.toMutableMap()

        for (word in words) {
            if (word != word.uppercase()) {
                println("$word!!!!!!!!!!!!!!!!!!!")
            }
            // Create groupings of all the characters in the word
            word.groupingBy { it }
                // Get counts of all the characters in the word
                .eachCount()
                // Add the counts to the overall counter
                .forEach { counter[it.key] = counter.getValue(it.key) + it.value }
        }

        val scores = words
            // Use only words with a unique set of characters (if desired)
            .filter { !uniqueOnly || it.toSet().size == it.length }
            // Get the score by summing the overall counter values of all the chars in the word
            .associateWith { word ->
                word.sumOf(counter::getValue) + if (word.toSet().size == word.length) 1000 else 0
            }

        val sortedWords = scores.asSequence()
            // Sort the words by score in reverse order
            .sortedBy { -it.value }.map { it.key }
            // Map the top-scoring words to their score
            .associateWith(scores::getValue)
        val topWords = mutableMapOf<String, Int>()

        // Figure out tiebreakers
        val topScore = sortedWords.asSequence().first().value
        // Get all the words which have the top score
        val tieBreaker = sortedWords.asSequence().takeWhile { it.value == topScore }
        if (tieBreaker.count() > 1) {
            // Calculate new scores based on how often each letter in the word will make a green square
            val greenScores = tieBreaker.map { it.key }.associateWith { tiedWord ->
                // Take the sum of all the times a letter in the tied word shows up in the same
                // index as any of the words in the overall word list
                words.sumOf { word ->
                    tiedWord.withIndex().count { word[it.index] == it.value }
                }
                // Sort by the scores
            }.asSequence().sortedBy { -it.value }
                // Turn the sequence into a map
                .associate { it.key to sortedWords.getValue(it.key) }
            topWords.putAll(greenScores)
        }
        return topWords +
            // For the remaining words, obviously ignore any that were in our tiebreaker
            sortedWords.asSequence().filterNot { topWords.contains(it.key) }
                // Only take enough words to fill out the requested n values
                .take((n - topWords.size).coerceAtLeast(0))
                // Create the map from the sequence we've been operating on
                .associate { it.key to it.value }
    }

    enum class Letter { BLACK, YELLOW, GREEN }

    /**
     * Determines one line of the wordle output
     */
    fun getResultOfGuess(guess: String, answer: String): List<Letter> {
        return guess.withIndex().map {
            when (it.value) {
                answer[it.index] -> Letter.GREEN
                in answer -> Letter.YELLOW
                else -> Letter.BLACK
            }
        }
    }

    /**
     * Play Wordle, going from a starting guess until finding the answer
     *
     * Returns the list of each guess, including the start and the answer
     */
    fun play(startingGuess: String, answer: String): List<String> {
        // These maps will track the result history of our guesses
        val yellow = mutableMapOf<Char, MutableList<Int>>()
        val green = mutableMapOf<Char, MutableList<Int>>()
        val black = mutableMapOf<Char, MutableList<Int>>()

        // The list of words guessed so far
        val guesses = mutableListOf<String>()
        // The current guess
        var guess = startingGuess
        // The remaining pool of words to guess from
        var words = this.wordList.toMutableList()
        // Handle old puzzles which might have different word lists
        if (!words.contains(answer)) {
            words += answer
        }

        do {
            val results = getResultOfGuess(guess, answer)
            for ((i, letter) in results.withIndex()) {
                // Record the results of the current guess
                when (letter) {
                    Letter.GREEN -> green.getOrPut(guess[i], ::mutableListOf).add(i)
                    Letter.YELLOW -> yellow.getOrPut(guess[i], ::mutableListOf).add(i)
                    else -> black.getOrPut(guess[i], ::mutableListOf).add(i)
                }
            }

            // Narrow down the pool of remaining words
            words = words.filter { word ->
                // Ensure no confirmed invalid letters are used
                word.withIndex().all { (i, c) ->
                    if (!green.contains(c)) {
                        return@all !black.contains(c)
                    } else {
                        return@all !(black[c]?.contains(i) ?: false)
                    }
                } &&
                    // Ensure that all yellow characters appear in the word
                    yellow.keys.all(word::contains) &&
                    // Ensure that we don't use a yellow letter in the same spot as before
                    yellow.asSequence().all { (c, indices) ->
                        indices.none { word[it] == c }
                    } &&
                    // Ensure any green characters are in the right places
                    green.asSequence().all { (c, indices) ->
                        indices.all { word[it] == c }
                    }
            }.toMutableList()

            guesses.add(guess)
            guess = findBestWords(words, n = 1).asSequence().first().key
        } while (guess != answer)

        return guesses + guess
    }

    /**
     * Get the Wordle answer from any given date
     */
    fun answer(date: LocalDate): String {
        val httpClient = HttpClient.newHttpClient()
        val request = HttpRequest.newBuilder(URI.create("$WORDLE_API_URL/$date.json")).build()
        val response = httpClient.send(request, HttpResponse.BodyHandlers.ofString())
        val json = response.body().trim()
        // Find where the desired index is declared
        return json.substringAfter("\"solution\":\"")
            // Cut off everything after the answer
            .substringBefore('"').uppercase()
    }

    companion object {
        private const val WORDLE_API_URL = "https://www.nytimes.com/svc/wordle/v2/"

        private const val WORD_LIST_URL = "https://gist.githubusercontent.com/dracos/dd0668f281e685bad51479e5acaadb93/raw/6bfa15d263d6d5b63840a8e5b64e04b382fdb079/valid-wordle-words.txt"

        private val log = Logger.getLogger("Wordle")
    }
}
