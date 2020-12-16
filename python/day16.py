import re
with open("../inputs/16.txt") as file:
    data = file.read()

# data = "\nclass: 1-3 or 5-7\nrow: 6-11 or 33-44\nseat: 13-40 or 45-50\n\nyour ticket:\n7,1,14\n\nnearby tickets:\n7,3,47\n40,4,50\n55,2,20\n38,6,12"

rules = re.findall(r"(\d+)-(\d+)", data)
acceptable_values = set(value for lower, upper in rules for value in range(int(lower), int(upper) + 1))
nearby_tickets = [[int(n) for n in ticket.split(',')] for ticket in data.split("tickets:\n")[-1].splitlines()]
invalid = [v for ticket in nearby_tickets for v in ticket if v not in acceptable_values]

print(f"Part 1: {sum(invalid)}")
