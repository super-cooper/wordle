/**
 * Finds the best word in the word list by counting the most common characters and then
 * scoring all the words with unique characters based on those counts.
 */
fun findBestWords(words: List<String>, n: Int = 5, uniqueOnly: Boolean = false): Map<String, Int> {
    if (words.size == 1) {
        return mapOf(words.first() to 0)
    }
    // Initialize counter of lowercase alphabet mapped to 0
    val counter = ('a'..'z').associateWith { 0 }.toMutableMap()

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
                yellow.keys.all { c ->
                    yellow.getValue(c).none { word[it] == c }
                } &&
                // Ensure any green characters are in the right places
                green.keys.all { c ->
                    green.getValue(c).all { word[it] == c }
                }
        }

        guesses.add(guess)
        guess = findBestWords(words, n = 1).asSequence().first().key
    } while (guess != answer)

    return guesses + guess
}
