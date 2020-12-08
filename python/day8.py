with open("../inputs/8.txt") as file:
    data = file.read().strip()

# data = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6"
code = [line.split(" ") for line in data.splitlines()]


def run(code_changed=-1):
    order = 1
    acc, ptr = 0, 0
    visited = [0 for line in code]
    while not visited[ptr]:
        visited[ptr] = order
        order += 1

        operation, argument = code[ptr]

        if order == code_changed:
            operation = "nop"

        if operation == "acc":
            acc += int(argument)
            ptr += 1
        if operation == "jmp":
            ptr += int(argument)
        if operation == "nop":
            ptr += 1

        if ptr > len(code) - 1:
            return True, acc

    return False, acc


_, acc = run()
print(f"Part 1: {acc}")

jmp_command_indexes = [i for i, c in enumerate(code) if c[0] == "jmp"]
finished, i = False, 0
while not finished:
    finished, acc = run(jmp_command_indexes[i])
    i += 1

print(f"Part 2: {acc}")
