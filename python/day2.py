import re

def valid1(rule):
    count = rule[3].count(rule[2])
    return count >= int(rule[0]) and count <= int(rule[1])

def valid2(rule):
    password = rule[3]
    char1 = password[int(rule[0])]
    char2 = password[int(rule[1])]
    expect = rule[2]
    return (char1==expect or char2==expect) and char1!=char2


with open("inputs/2.txt") as file:
    data = file.read()
# data = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc"

rules = re.findall(r"(\d+)-(\d+) (.):(.*)", data)

part1 = sum(1 for x in rules if valid1(x))
print(f"Day 2 part 1: {part1}")
part2 = sum(1 for x in rules if valid2(x))
print(f"Day 2 part 2: {part2}")
