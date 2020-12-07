with open("../inputs/6.txt") as file:
    data = file.read().strip()
# data = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb"

groups = data.split("\n\n")

part1 = sum(len(set([answer for person in group.splitlines() for answer in person])) for group in groups)
print(part1)

answers_sets = [[set(answer) for answer in group.splitlines()] for group in groups]
all_answered = 0
for s in answers_sets:
    all_answered += len(s[0].intersection(*s[1:]))
print(all_answered)
