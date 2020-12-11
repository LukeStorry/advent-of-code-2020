from typing import List

floor, empty, occupied = '.', 'L', '#'

with open("../inputs/11.txt") as file:
    data = file.read().strip()

# data = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL"


def new_seats_adjacent(x: int, y: int, layout: List[str]) -> str:
    if layout[y][x] == floor:
        return floor
    neighbours_coords = [(y - 1, x - 1), (y - 1, x), (y - 1, x + 1),
                         (y, x - 1), (y, x + 1),
                         (y + 1, x - 1), (y + 1, x), (y + 1, x + 1)]

    neighbours = (layout[_y][_x] for (_y, _x) in neighbours_coords if
                  0 <= _x < len(layout[0]) and 0 <= _y < len(layout))
    occupied_neighbours = sum(1 for neighbor in neighbours if neighbor == occupied)
    if layout[y][x] == empty and occupied_neighbours == 0:
        return occupied
    if layout[y][x] == occupied and occupied_neighbours >= 4:
        return empty

    return layout[y][x]


def new_state_visible(x: int, y: int, layout: List[str]) -> str:
    if layout[y][x] == floor:
        return floor
    width, height = len(layout[0]), len(layout)
    edge = max(width, height)
    n = ((y - i, x) for i in range(1, y + 1))
    s = ((y + i, x) for i in range(1, height - y))
    e = ((y, x + i) for i in range(1, width - x))
    w = ((y, x - i) for i in range(1, x + 1))
    ne = ((y - i, x + i) for i in range(1, edge) if y - i >= 0 and x + i < width)
    se = ((y + i, x + i) for i in range(1, edge) if y + i < height and x + i < width)
    sw = ((y + i, x - i) for i in range(1, edge) if y + i < height and x - i >= 0)
    nw = ((y - i, x - i) for i in range(1, edge) if y - i >= 0 and x - i >= 0)

    visible = (next((layout[_y][_x] for (_y, _x) in direction if layout[_y][_x] != floor), '') for direction in
               (n, s, e, w, ne, se, sw, nw))

    occupied_visible = sum(1 for seat in visible if seat == occupied)

    if layout[y][x] == empty and occupied_visible == 0:
        return occupied
    if layout[y][x] == occupied and occupied_visible >= 5:
        return empty

    return layout[y][x]


# Part 1
seats = data.splitlines()

while True:
    next_seats = [[new_seats_adjacent(x, y, seats) for x in range(len(seats[0]))] for y in range(len(seats))]
    if next_seats == seats:
        break
    seats = next_seats

print(f"Day 11 Part 1: {[s for line in seats for s in line].count('#')}")

# Part 2
seats = data.splitlines()

while True:
    next_seats = [[new_state_visible(x, y, seats) for x in range(len(seats[0]))] for y in range(len(seats))]
    if next_seats == seats:
        break
    seats = next_seats

print(f"Day 11 Part 2: {[s for line in seats for s in line].count('#')}")
