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

    val wordle = Wordle()

    when (args[0]) {
        "top" -> {
            if (args.size != 1) {
                errorExit()
            }
            val top5Words = wordle.findBestWords(uniqueOnly = true)
            println(
                "Top 5 words:\n${
                top5Words.asSequence()
                    .joinToString(separator = "\n") { "${it.key} ${it.value}" }
                }"
            )
        }

        "play" -> {
            if (args.size != 2) {
                errorExit()
            }
            val answer = args[1].uppercase()
            val board = wordle.play(wordle.bestWord, answer)
            println(board.joinToString(separator = "\n"))
        }

        "list" -> {
            if (args.size != 1) {
                errorExit()
            }
            println(wordle.wordList.joinToString(separator = "\n"))
        }

        "answer" -> {
            if (args.size > 2) {
                errorExit()
            }
            println(wordle.answer(args.getOrNull(1)?.toInt() ?: wordle.count))
        }

        "count" -> {
            if (args.size != 1) {
                errorExit()
            }
            println(wordle.count)
        }
    }
}
