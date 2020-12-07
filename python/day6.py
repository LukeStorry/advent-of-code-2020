with open("../inputs/6.txt") as file:
    data = file.read().strip()
# data = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb"
groups = data.split("\n\n")

group_sets = [set(answer for person in group.splitlines() for answer in person) for group in groups]
part1 = sum(len(s) for s in group_sets)
print(part1)

answers_sets = [[set(answer) for answer in group.splitlines()] for group in groups]
part2 = sum(len(s[0].intersection(*s[1:])) for s in answers_sets)
print(part2)
