# Wordle

These are some simple scripts I wrote to explore the [Wordle](https://www.nytimes.com/games/wordle/) game. 

* [best_word.py](./best_word.py) - Tries to determine the best starting word in wordle by scoring words based on how common each of their letters are in the wordle dictionary.
* [greedy.py](./greedy.py) - Greedy Wordle-solving script. Takes in game state as input, and then uses the same scoring algorithm as [best_word.py](./best_word.py) on the available set of valid words, which is recalculated for every turn.
