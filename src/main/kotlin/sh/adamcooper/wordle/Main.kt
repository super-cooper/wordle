package sh.adamcooper.wordle

import kotlin.system.exitProcess

private fun errorExit() {
    println("Valid commands: \ntop\nplay <answer>\nlist\nanswer [n]\ncount")
    exitProcess(1)
}

fun main(args: Array<String>) {
    if (args.isEmpty()) {
        errorExit()
    }

    val words = downloadWordList()

    when (args[0]) {
        "top" -> {
            if (args.size != 1) {
                errorExit()
            }
            val top5Words = findBestWords(words, uniqueOnly = true)
            println(
                "Top 5 words:\n${
                top5Words.asSequence()
                    .joinToString(separator = "\n") { "${it.key} ${it.value}" }
                }"
            )
        }

        "play" -> {
            if (args.size > 2) {
                errorExit()
            }
            val answer = (args.getOrNull(1) ?: wordleAnswer(wordleCount())).uppercase()
            val board = playWordle(
                words,
                findBestWords(words, n = 1, uniqueOnly = true).asSequence().first().key,
                answer
            )
            println(board.joinToString(separator = "\n"))
        }

        "list" -> {
            if (args.size != 1) {
                errorExit()
            }
            println(words.joinToString(separator = "\n"))
        }

        "answer" -> {
            if (args.size > 2) {
                errorExit()
            }
            println(wordleAnswer(args.getOrNull(1)?.toInt() ?: wordleCount()))
        }

        "count" -> {
            if (args.size != 1) {
                errorExit()
            }
            println(wordleCount())
        }
    }
}
