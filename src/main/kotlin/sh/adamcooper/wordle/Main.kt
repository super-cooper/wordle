package sh.adamcooper.wordle

import java.io.File
import kotlin.system.exitProcess

private fun errorExit() {
    println("Valid commands: \ntop\nplay <answer>\nlist\nanswer [n]")
    exitProcess(1)
}

fun main(args: Array<String>) {
    if (args.isEmpty()) {
        errorExit()
    }

    val words = File("words.txt").readText().split(",").map { it.trim() }

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
            if (args.size != 2) {
                errorExit()
            }
            val answer = args[1]
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
            println(downloadWordList().joinToString(separator = "\n"))
        }

        "answer" -> {
            if (args.size != 2) {
                errorExit()
            }
            println(getWordleAnswer(args[1].toInt()))
        }
    }
}
