with open("../inputs/8.txt") as file:
    data = file.read().strip()

# data = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6"
code = [line.split(" ") for line in data.splitlines()]

acc, ptr = 0, 0
visited = [False for line in code]
while not visited[ptr]:
    visited[ptr] = True
    operation, argument = code[ptr]
    if operation == "acc":
        acc += int(argument)
        ptr += 1
    if operation == "jmp":
        ptr += int(argument)
    if operation == "nop":
        ptr += 1

print(acc)
