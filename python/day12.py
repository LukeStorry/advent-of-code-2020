import math

with open("../inputs/12.txt") as file:
    data = file.read().strip()

# data = "F10\nN3\nF7\nR90\nF11"
actions = data.splitlines()

x, y, d = 0, 0, 0
for action in actions:
    nav, amount = action[0], int(action[1:])
    if nav == 'N':
        y += amount
    elif nav == 'S':
        y -= amount
    elif nav == 'E':
        x += amount
    elif nav == 'W':
        x -= amount
    elif nav == 'L':
        d -= amount
    elif nav == 'R':
        d += amount
    elif nav == 'F':
        x += amount * int(math.cos(math.radians(d)))
        y -= amount * int(math.sin(math.radians(d)))

print(f"Part 1: {abs(x) + abs(y)}")  # 1631
