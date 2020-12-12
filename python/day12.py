import math

with open("../inputs/12.txt") as file:
    data = file.read().strip()

# data = "F10\nN3\nF7\nR90\nF11"
instructions = [(line[0], int(line[1:])) for line in data.splitlines()]

x, y, d = 0, 0, 0
for action, amount in instructions:
    if action == 'N':
        y += amount
    elif action == 'S':
        y -= amount
    elif action == 'E':
        x += amount
    elif action == 'W':
        x -= amount
    elif action == 'L':
        d -= amount
    elif action == 'R':
        d += amount
    elif action == 'F':
        x += amount * int(math.cos(math.radians(d)))
        y -= amount * int(math.sin(math.radians(d)))

print(f"Part 1: {abs(x) + abs(y)}")  # 1631

ship_x, ship_y, waypoint_x, waypoint_y = 0, 0, 10, 1
for action, amount in instructions:
    if action == 'N':
        waypoint_y += amount
    elif action == 'S':
        waypoint_y -= amount
    elif action == 'E':
        waypoint_x += amount
    elif action == 'W':
        waypoint_x -= amount
    elif action == 'L':
        while amount > 0:
            amount -= 90
            waypoint_x, waypoint_y = -waypoint_y, waypoint_x
    elif action == 'R':
        while amount > 0:
            amount -= 90
            waypoint_x, waypoint_y = waypoint_y, -waypoint_x
    elif action == 'F':
        ship_x, ship_y = ship_x + waypoint_x * amount, ship_y + waypoint_y * amount

print(f"Part 2: {abs(ship_x) + abs(ship_y)}")  # 58606
