import re
from typing import Optional

re_equation_in_brackets = re.compile(r"\(([\d +*]+)\)")
re_number_in_brackets = re.compile(r"\((\d+)\)")
re_expression = re.compile(r"(?=(\d+) ([+*]) (\d+))")


def next_expression(string: str, advanced, start=0, end=9999999999) -> Optional[re.Match]:
    brackets = re_equation_in_brackets.search(string, pos=start, endpos=end)
    if brackets is not None:
        return next_expression(string, advanced, brackets.start(1), brackets.end(1))

    expressions = list(re_expression.finditer(string, pos=start, endpos=end))
    if not len(expressions):
        return None
    if advanced:
        for expression in expressions:
            if expression.group(2) == "+":
                return expression

    return expressions[0]


def solve(string: str, advanced=False) -> int:
    expression = next_expression(string, advanced)

    while expression is not None:
        first, operator, second = expression.groups()
        replacement = int(first) + int(second) if operator == "+" else int(first) * int(second)
        before = string[:expression.start(1)]
        after = string[expression.end(3):]
        string = before + str(replacement) + after
        string = re_number_in_brackets.sub(r"\1", string)
        expression = next_expression(string, advanced)

    return int(string)


assert (solve("2 * 3 + (4 * 5)") == 26)
assert (solve("5 + (8 * 3 + 9 + 3 * 4 * 3)") == 437)
assert (solve("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))") == 12240)
assert (solve("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2") == 13632)

assert (solve("2 * 3 + (4 * 5)", True) == 46)
assert (solve("5 + (8 * 3 + 9 + 3 * 4 * 3)", True) == 1445)
assert (solve("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", True) == 669060)
assert (solve("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", True) == 23340)

with open("../inputs/18.txt") as file:
    data = file.read()
part_1 = sum(map(solve, data.splitlines()))
print(f"Part 1: {part_1}")

part_2 = sum(map(lambda x: solve(x, True), data.splitlines()))
print(f"Part 2: {part_2}")  # 3348222486398
