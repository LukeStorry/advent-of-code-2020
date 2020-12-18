import re

re_brackets = re.compile(r"\((\d+)\)")
re_expression = re.compile(r"(\d+) ([+*]) (\d+)")


def solve(string: str) -> int:
    expression = re_expression.search(string)
    while expression is not None:
        first, operator, second = expression.groups()
        replacement = int(first) + int(second) if operator == "+" else int(first) * int(second)
        string = string[:expression.start()] + str(replacement) + string[expression.end():]
        string = re_brackets.sub(r"\1", string)
        expression = re_expression.search(string)

    return int(string)


# assert (solve("2 * 3 + (4 * 5)") == 26)
# assert (solve("5 + (8 * 3 + 9 + 3 * 4 * 3)") == 437)
# assert (solve("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))") == 12240)
# assert (solve("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2") == 13632)

with open("../inputs/18.txt") as file:
    data = file.read()
part_1 = sum(map(solve, data.splitlines()))
print(f"Part 1: {part_1}")
