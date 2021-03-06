with open("../inputs/10.txt") as file:
    data = file.read().strip()
# data = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4"
# data = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3"

numbers = [int(i) for i in data.splitlines()]

outlet, device = 0, max(numbers) + 3
joltages = [outlet] + sorted(numbers) + [device]
diffs = [a - b for a, b in zip(joltages[1:], joltages)]
print(f"Day 10 Part 1: {diffs.count(1) * diffs.count(3)}")

results = [0 for _ in range(device + 1)]
results[0] = 1
for joltage in joltages:
    results[joltage] += results[joltage - 1] + results[joltage - 2] + results[joltage - 3]

print(f"Day 10 Part 2: {results[-1]}")
