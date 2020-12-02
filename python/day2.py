import re

def valid1(match):
    count = match[3].count(match[2])
    return count >= int(match[0]) and count <= int(match[1])

def valid2(match):
    password = match[3]
    char1 = password[int(match[0])]
    char2 = password[int(match[1])]
    expect = match[2]
    return (char1==expect or char2==expect) and char1!=char2


with open("inputs/2.txt") as file:
    data = file.read()
# data = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc"

rules = re.findall(r"(\d*)-(\d*) (.):(.*)", data)

part1 = sum(1 for x in rules if valid1(x))
print(f"Day 2 part 1: {part1}")
part2 = sum(1 for x in rules if valid2(x))
print(f"Day 2 part 2: {part2}")
