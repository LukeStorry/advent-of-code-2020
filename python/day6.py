from typing import Set

with open("../inputs/6.txt") as file:
    data = file.read().strip()
# data = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb"

count = [len(set(answer for person in group.splitlines() for answer in person)) for group in (data.split("\n\n"))]

print(sum(count)) # 6170
