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

# part 2
valid_messages_for_31 = generate_valid_messages(rules['31'])
valid_messages_for_42 = generate_valid_messages(rules['42'])

# print(len(valid_messages_for_31))
# print(len(valid_messages_for_42))
# print(set(len(x) for x in valid_messages_for_31))
# print(set(len(x) for x in valid_messages_for_42))
# >> all valid messages in there are 8 chars long

count = 0
for message in messages:
    sections = [message[i:i + 8] for i in range(0, len(message), 8)]
    check_string = ''.join(str(section in valid_messages_for_42)[0] for section in sections)

    if check_string.count('T') > check_string.count('F') and 'TF' in check_string and 'FT' not in check_string:
        count += 1

print(count)
