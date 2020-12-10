with open("../inputs/10.txt") as file:
    data = file.read().strip()
# data = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4"
# data = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3"

numbers = [int(i) for i in data.splitlines()]
outlet, adapter = [0], [max(numbers) + 3]
chain = outlet + sorted(numbers) + adapter
# print(chain)
diffs = [a - b for a, b in zip(chain[1:], chain)]
print(diffs.count(1) * diffs.count(3))
