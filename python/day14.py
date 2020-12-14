import re

with open("../inputs/14.txt") as file:
    data = file.read().strip()
# data = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0"
# data = "mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1"

program = re.findall(r'(.+) = (.+)', data)

mask_ones, mask_zeros, mem = 0, 0, {}
for instruction, value in program:
    if instruction == "mask":
        mask_zeros = int(value.replace('X', '1'), 2)
        mask_ones = int(value.replace('X', '0'), 2)
    else:
        mem[instruction[4:-1]] = int(value) & mask_zeros | mask_ones

print(f"Part 1: {sum(mem.values())}")

mask_ones, mask_x_indexes, mem = 0, [], {}
for instruction, value in program:
    if instruction == 'mask':
        mask_ones = int(value.replace('X', '0'), 2)
        mask_x_indexes = [index for index, char in enumerate(reversed(value)) if char == 'X']
    else:
        locations = [int(instruction[4:-1]) | mask_ones]
        for i in mask_x_indexes:
            locations = [new_location for old_location in locations for new_location in
                         (old_location & ~(1 << i), old_location | (1 << i))]

        for location in locations:
            mem[location] = int(value)

print(f"Part 2: {sum(mem.values())}")
