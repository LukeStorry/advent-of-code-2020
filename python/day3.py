import re

with open("inputs/3.txt") as file:
    data = file.read()

map = data.split("\n")

def is_tree(x: int, y: int) -> bool:
    line = map[y]
    char = line[x % len(line)]
    return char == "#"

trees = sum(is_tree(y * 3, y) for y in range(1, len(map)))
print(trees)
