
import math
def binary_split(s: str, max: int, lower_char: str) -> int:
    lower, upper, new = 0, max, -1
    for ch in s:
        if ch == lower_char:
            upper = math.floor((upper - lower) / 2) + lower
        else:
            lower = math.ceil((upper - lower) / 2) + lower
    return int(lower)

def get_seat(boarding_pass:str)-> int:
    row, col = binary_split(boarding_pass[:-3], 127, "F"), binary_split(boarding_pass[-3:], 7, "L")
    return row*8+col

with open("../inputs/5.txt") as file:
    data = file.read().strip().splitlines()

seats = [get_seat(x) for x in data]
max_seat = max(seats)
print(f"Part 1: {max_seat}")

unused = set(range(min(seats) + 1, max_seat - 1)) - set(seats)
print(f"Part 2: {unused}")
