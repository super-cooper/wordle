package sh.adamcooper.wordle

import java.io.BufferedReader
import java.io.InputStreamReader
import java.net.URL

private val ANSWER_LIST_URL = URL("https://wordfinder.yourdictionary.com/wordle/answers/")
private val JSON_PAIR_REGEX = Regex(""".*JSON\.parse\((.+)""")

private val WORD_LIST_URL = URL("https://wordletoday.org/wordle-words.php")
private val WORD_LIST_SCRAPING_REGEX = Regex("""^\s*([a-z]+(,\s|</p>))+\s*$""")

/**
 * Finds the best word in the word list by counting the most common characters and then
 * scoring all the words with unique characters based on those counts.
 */
fun findBestWords(words: List<String>, n: Int = 5, uniqueOnly: Boolean = false): Map<String, Int> {
    if (words.size == 1) {
        return mapOf(words.first() to 0)
    }
    // Initialize counter of lowercase alphabet mapped to 0
    val counter = ('A'..'Z').associateWith { 0 }.toMutableMap()

    for (word in words) {
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
            word.sumOf(counter::getValue)
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
fun playWordle(wordList: List<String>, startingGuess: String, answer: String): List<String> {
    // These maps will track the result history of our guesses
    val yellow = mutableMapOf<Char, MutableList<Int>>()
    val green = mutableMapOf<Char, MutableList<Int>>()
    val black = mutableMapOf<Char, MutableList<Int>>()

    // The list of words guessed so far
    val guesses = mutableListOf<String>()
    // The current guess
    var guess = startingGuess
    // The remaining pool of words to guess from
    var words = wordList.toList()

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
        }

        guesses.add(guess)
        guess = findBestWords(words, n = 1).asSequence().first().key
    } while (guess != answer)

    return guesses + guess
}

/**
 * Downloads the list of all possible answers for Wordle.
 */
fun downloadWordList(): List<String> {
    val reader = BufferedReader(InputStreamReader(WORD_LIST_URL.openStream()))
    return buildList {
        for (line in reader.lineSequence()) {
            if (line.matches(WORD_LIST_SCRAPING_REGEX)) {
                this.addAll(
                    line.replace("</p>", "")
                        .splitToSequence(',')
                        .map(String::trim)
                        .map(String::uppercase)
                        .filter(String::isNotEmpty)
                )
            }
        }
    }.also {
        reader.close()
    }
}

private fun wordleScrapedJSON(): String {
    val reader = BufferedReader(InputStreamReader(ANSWER_LIST_URL.openStream()))

    // Find the line where the answers JSON object exists
    val matchLine = JSON_PAIR_REGEX.find(reader.readText())?.groups?.find {
        it?.value?.startsWith('"') ?: false
    }?.value ?: throw IllegalArgumentException("Could not find line with answers")
    // Pull the raw JSON out of the call to `JSON.parse`
    return matchLine.substring(0..matchLine.indexOf("}\");")).trimStart('"').replace("\\", "")
}

/**
 * Get the Wordle answer from any given date
 */
fun wordleAnswer(index: Int): String {
    // Find where the desired index is declared
    return wordleScrapedJSON().substringAfter("\"index\":$index,")
        // Find where the answer is declared after the index
        .substringAfter("\"answer\":\"")
        // Cut off everything after the answer
        .substringBefore('"')
}

/**
 * Find out the number of wordle puzzles that have been posted
 */
fun wordleCount(): Int {
    // Subtract 2 to remove the splits before the first occurrence of "index" and after the last
    return wordleScrapedJSON().splitToSequence(Regex(""""index":""")).count() - 2
}
