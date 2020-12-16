import math
import re

with open("../inputs/16.txt") as file:
    data = file.read()

# data = "\nclass: 1-3 or 5-7\nrow: 6-11 or 33-44\nseat: 13-40 or 45-50\n\nyour ticket:\n7,1,14\n\nnearby tickets:\n7,3,47\n40,4,50\n55,2,20\n38,6,12"
# data = "class: 0-1 or 4-19\nrow: 0-5 or 8-19\nseat: 0-13 or 16-19\n\nyour ticket:\n11,12,13\n\nnearby tickets:\n3,9,18\n15,1,5\n5,14,9"

your_ticket = [int(n) for n in data.split("ticket:\n")[1].splitlines()[0].split(',')]
nearby_tickets = [[int(n) for n in ticket.split(',')] for ticket in data.split("tickets:\n")[-1].splitlines()]

fields = {name: set(list(range(int(a), int(b) + 1)) + list(range(int(c), int(d) + 1)))
          for name, a, b, c, d in re.findall(r"([\w ]+): (\d+)-(\d+) or (\d+)-(\d+)", data)}

all_valid_numbers = set(v for s in fields.values() for v in s)
invalid_values = [v for ticket in nearby_tickets for v in ticket if v not in all_valid_numbers]

print(f"Part 1: {sum(invalid_values)}")

valid_nearby_tickets = [ticket for ticket in nearby_tickets if
                        not any(value not in all_valid_numbers for value in ticket)]

field_possible_indexes = {name: set(range(len(fields))) for name in fields}
for ticket in valid_nearby_tickets:
    for index, value in enumerate(ticket):
        for field_name in field_possible_indexes:
            if index in field_possible_indexes[field_name] and value not in fields[field_name]:
                field_possible_indexes[field_name].remove(index)

known_indexes = []
while len(known_indexes) < len(fields):
    for field, possible_indexes in field_possible_indexes.items():
        if len(possible_indexes) == 1:
            index = possible_indexes.pop()
            known_indexes.append((index, field))
            for other in (possibility for possibility in field_possible_indexes.values() if index in possibility):
                other.remove(index)

departure_values = [your_ticket[i] for i, name in known_indexes if "departure" in name]
print(f"Part 2: {math.prod(departure_values)}")
