with open("../inputs/19.txt") as file:
    data = file.read()

# data = '0: 4 1 5\n1: 2 3 | 3 2\n2: 4 4 | 5 5\n3: 4 5 | 5 4\n4: "a"\n5: "b"\n\nababbb\nbababa\nabbbab\naaabbb\naaaabbb'

input_rules, messages = (x.splitlines() for x in data.split('\n\n'))

rules = dict(rule.split(": ") for rule in input_rules)


def generate_valid_messages(rule: str) -> set:
    if '"' in rule:
        return set(rule[1])

    if '|' in rule:
        return set(m for r in rule.split(' | ') for m in generate_valid_messages(r))

    if ' ' in rule:
        left, right = [generate_valid_messages(x) for x in rule.split(' ', 1)]
        return set(x + y for x in left for y in right)

    return generate_valid_messages(rules[rule])


all_valid_messages = generate_valid_messages(rules['0'])
print(sum(message in all_valid_messages for message in messages))

rules['8'] = "42 | 42 8"
rules['11'] = "42 31 | 42 11 31"

all_valid_messages = generate_valid_messages(rules['0'])
print(sum(message in all_valid_messages for message in messages))
