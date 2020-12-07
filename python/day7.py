import re
from collections import deque

with open("../inputs/7.txt") as file:
    data = file.read().strip()
# data = "light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags."

line_regex = re.compile(r'(\w+ \w+) bags contain (.*)')
contents_regex = re.compile(r'(\d+) (\w+ \w+) bags?')

bags = {}
for line in data.splitlines():
    bag_name, contents = re.match(line_regex, line).groups()
    bags[bag_name] = [(int(contents_match.group(1)), contents_match.group(2)) for contents_match in
                      re.finditer(contents_regex, contents)]


def contents_generator(bag):
    queue = deque(bags[bag])
    while queue:
        count, contents = queue.popleft()
        yield count, contents
        queue.extend((count * subcount, subcontents) for subcount, subcontents in bags[contents])


contents = {bag: list(contents_generator(bag)) for bag in bags}

print(sum(any(bag == 'shiny gold' for _, bag in bag_queues) for bag_queues in contents.values()))
print(sum(count for count, _ in contents['shiny gold']))
