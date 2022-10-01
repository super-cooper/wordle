/**
 * Finds the best word in the word list by counting the most common characters and then
 * scoring all the words with unique characters based on those counts.
 */
fun findBestWords(words: List<String>, n: Int = 5): Map<String, Int> {
    // Initialize counter of lowercase alphabet mapped to 0
    val counter = ('a'..'z').associateWith { 0 }.toMutableMap()

    for (word in words) {
        // Create groupings of all the characters in the word
        word.groupingBy { it }
            // Get counts of all the characters in the word
            .eachCount()
            // Add the counts to the overall counter
            .forEach { (letter, count) ->
                counter[letter] = counter.getValue(letter) + count
            }
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
