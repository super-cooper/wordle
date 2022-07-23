#!/bin/env python3

import collections
import enum
import string

with open("words.txt", "r") as f:
    words = {word.strip() for word in f.read().split(",")}


def score_words(guesses: set[str]) -> dict[str, int]:
    counter = collections.Counter({c: 0 for c in string.ascii_lowercase})

    # Count most common characters
    for word in guesses:
        counter += collections.Counter(word)

    # Score words
    return {
        # Cumulative scoring based on frequency of each letter
        word: sum(counter[c] for c in word)
        # A Million point bonus if all letters are unique, to prioritize this property
        + (1_000_000 if len(set(word)) == len(word) else 0)
        for word in guesses
    }


class Letter(enum.IntEnum):
    BLACK = 0
    YELLOW = 1
    GREEN = 2


yellow = collections.defaultdict(list)
green = collections.defaultdict(list)
black = set()

word = input("Word: ")
results = [Letter.BLACK] * 5
while results != [Letter.GREEN] * 5:
    results = [Letter(int(c)) for c in input("Results: ")]

    for i, (c, result) in enumerate(zip(word, results)):
        # Gather new state on letters used in this guess
        if result == Letter.GREEN:
            green[c].append(i)
        elif result == Letter.YELLOW:
            yellow[c].append(i)
        elif result == Letter.BLACK:
            black.add(c)

    # Narrow space of potential words to adhere to game state
    words = {
        w
        for w in words
        # Ensure no invalid letters are used
        if all(c not in black for c in w)
        # Ensure we don't use a yellow letter in the same spot as before
        and all(i not in yellow.get(c, []) for i, c in enumerate(w))
        # Ensure any green characters are in the right places
        and all(i in green.get(c, [i]) for i, c in enumerate(w))
        # Ensure that all yellow characters appear in the word
        and any(c in (yellow or [c]) for c in w)
    }

    # Recalculate scores for new set of words
    scores = score_words(words)

    # Find the word with the maximum score
    word = max(words, key=lambda w: scores[w])
    print(word)

print("Done.")
