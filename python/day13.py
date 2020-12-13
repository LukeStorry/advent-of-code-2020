with open("../inputs/13.txt") as file:
    data = file.read().strip()

# data = "939\n7,13,x,x,59,x,31,19"

_time, _busses = data.splitlines()
current_time, busses = int(_time), [int(bus) for bus in _busses.split(",") if bus != 'x']
next_bus, next_bus_id = min((bus * (current_time // bus + 1), bus) for bus in busses)
print(f"Part 1: {(next_bus - current_time) * next_bus_id}")


requirements = [(int(bus_id), offset) for offset, bus_id in enumerate(_busses.split(',')) if bus_id != 'x']
delta, time = 1, 10000

for bus, offset in requirements:
    while (time + offset) % bus != 0:
        time += delta
    delta *= bus

print(f"Part 2: {time}")
