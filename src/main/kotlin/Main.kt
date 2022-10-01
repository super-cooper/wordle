import java.io.File
import kotlin.system.exitProcess

fun main(args: Array<String>) {
    if (args.size > 1) {
        println("Valid commands: [top]")
        exitProcess(1)
    }

    val words = File("words.txt").readText().split(",").map { it.trim() }
    when (args[0]) {
        "top" -> {
            val top5Words = findBestWords(words)
            println(
                "Top 5 words:\n${
                top5Words.asSequence()
                    .joinToString(separator = "\n") { "${it.key} ${it.value}" }
                }"
            )
        }
    }
}
