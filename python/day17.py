with open("../inputs/17.txt") as file:
    data = file.read()
# data = ".#.\n..#\n###"

actives = set((x, y, 0) for (y, line) in enumerate(data.splitlines()) for (x, char) in enumerate(line) if char == '#')

def neighbours(x, y, z):
    return [(_x, _y, _z)
            for _x in (x - 1, x, x + 1)
            for _y in (y - 1, y, y + 1)
            for _z in (z - 1, z, z + 1)
            if (_x, _y, _z) != (x, y, z)]


def print_pocket(coords):
    min_x, max_x = min(x for (x, y, z) in coords) - 1, max(x for (x, y, z) in coords) + 2
    min_y, max_y = min(y for (x, y, z) in coords) - 1, max(y for (x, y, z) in coords) + 2
    min_z, max_z = min(z for (x, y, z) in coords), max(z for (x, y, z) in coords) + 1
    for z in range(min_z, max_z):
        print(f"z={z}")
        for y in range(min_y, max_y):
            print(''.join('#' if (x, y, z) in coords else '.' for x in range(min_x, max_x)))


for cycle in range(6):
    # print(f"After {cycle} cycles:")
    # print_pocket(actives)
    new_actives = set()
    all_coords = set(list(actives) + [n for active in actives for n in neighbours(*active)])
    for cube in all_coords:
        active_neighbours = sum(neighbour in actives for neighbour in neighbours(*cube))
        if cube in actives and active_neighbours in [2, 3]:
            new_actives.add(cube)
        if cube not in actives and active_neighbours == 3:
            new_actives.add(cube)
    actives = new_actives

print(f"Part 1: {len(actives)}")
