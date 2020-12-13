with open("../inputs/13.txt") as file:
    data = file.read().strip()

data = "939\n7,13,x,x,59,x,31,19"

_time, _busses = data.splitlines()
current_time, busses = int(_time), [int(bus) for bus in _busses.split(",") if bus != 'x']
next_bus, next_bus_id = min((bus * (current_time // bus + 1), bus) for bus in busses)
print(f"Part 1: {(next_bus - current_time) * next_bus_id}")

t, requirements = 0, []
for bus in _busses.split(","):
    if bus != 'x':
        requirements.append((int(bus), t))
    t += 1

time = 10000
print(requirements)

while True:
    if any((time + offset) % bus for (bus, offset) in requirements):
        time += 1
    else:
        break
print(f"Part 2: {time}")
