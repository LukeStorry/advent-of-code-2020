with open("inputs/1.txt") as file:
    data = file.read()
nums = list(map(int, data.split("\n")))

part1 = next((2020 - num) * num for num in nums if (2020 - num) in nums)
print(f"Day 2 part 1: {part1}")

part2 = next((2020 - x - y) * x * y for y in nums for x in nums if (2020 - x - y) in nums)
print(f"Day 2 part 2: {part2}")
