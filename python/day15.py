data = "0,13,1,16,6,17"
numbers = [int(n) for n in data.split(',')]

last_spoken = {n: i for i, n in enumerate(numbers, start=1)}
last_number = 0
for turn in range(len(numbers) + 1, 30000000):
    if last_number in last_spoken:
        next_number = turn - last_spoken[last_number]
    else:
        next_number = 0
    last_spoken[last_number] = turn
    last_number = next_number
    if turn == 2019:
        print(f"Part 1: {last_number}")
print(f"Part 2: {last_number}")
