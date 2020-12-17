with open("../inputs/17.txt") as file:
    data = file.read()

# data = ".#.\n..#\n###"


def neighbours_3d(x, y, z):
    return [(_x, _y, _z)
            for _x in (x - 1, x, x + 1)
            for _y in (y - 1, y, y + 1)
            for _z in (z - 1, z, z + 1)
            if (_x, _y, _z) != (x, y, z)]


def neighbours_4d(w, x, y, z):
    return [(_w, _x, _y, _z)
            for _w in (w - 1, w, w + 1)
            for _x in (x - 1, x, x + 1)
            for _y in (y - 1, y, y + 1)
            for _z in (z - 1, z, z + 1)
            if (_w, _x, _y, _z) != (w, x, y, z)]


def solve(actives, neighbours_fn):
    for cycle in range(6):
        new_actives = set()
        all_coords = set(list(actives) + [n for active in actives for n in neighbours_fn(*active)])
        for cube in all_coords:
            active_neighbours = sum(neighbour in actives for neighbour in neighbours_fn(*cube))
            if cube in actives and active_neighbours in [2, 3]:
                new_actives.add(cube)
            if cube not in actives and active_neighbours == 3:
                new_actives.add(cube)
        actives = new_actives
    return len(actives)


initial_1 = set((x, y, 0) for (y, line) in enumerate(data.splitlines()) for (x, char) in enumerate(line) if char == '#')
print(f"Part 1: {solve(initial_1, neighbours_3d)}")

initial_2 = set(
    (x, y, 0, 0) for (y, line) in enumerate(data.splitlines()) for (x, char) in enumerate(line) if char == '#')
print(f"Part 2: {solve(initial_2, neighbours_4d)}")
