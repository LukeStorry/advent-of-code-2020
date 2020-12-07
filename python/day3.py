with open("inputs/3.txt") as file:
    data = file.read()
map = data.split("\n")
# map = ["..##.......","#...#...#..",".#....#..#.","..#.#...#.#",".#...##..#.","..#.##.....",".#.#.#....#",".#........#","#.##...#...","#...##....#",".#..#...#.#"]


def is_tree(x: int, y: int) -> bool:
    line = map[y]
    char = line[x % len(line)]
    return char == "#"


slope1 = sum(is_tree(y, y) for y in range(1, len(map)))
slope2 = sum(is_tree(y * 3, y) for y in range(1, len(map)))
slope3 = sum(is_tree(y * 5, y) for y in range(1, len(map)))
slope4 = sum(is_tree(y * 7, y) for y in range(1, len(map)))
slope5 = sum(is_tree(y // 2, y) for y in range(0, len(map), 2))

print(f"part 1: {slope2}")
print(f"part 2: {slope1 * slope2 * slope3 * slope4 * slope5}")

