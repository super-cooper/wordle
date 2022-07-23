#!/bin/env python3

import collections
import string

with open("words.txt", 'r') as f:
    words = [word.strip() for word in f.read().split(",")]

counter = collections.Counter({c: 0 for c in string.ascii_lowercase})

# Count most common characters
for word in words:
    counter += collections.Counter(word)

print(f"Most common characters:\n{counter.most_common()}")

# Score only words with all unique characters
scores = {word: sum(counter[c] for c in word) for word in words if len(set(word)) == len(word)}
top5_words = sorted(scores, reverse=True, key=lambda w: scores[w])[:5]

result_string = "\n".join(f'{word} {scores[word]}' for word in top5_words)
print(f"Top 5 words:\n{result_string}")

