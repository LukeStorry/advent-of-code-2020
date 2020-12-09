from itertools import permutations

with open("../inputs/9.txt") as file:
    data = file.read().strip()
# data = "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576"

numbers = [int(i) for i in data.splitlines()]


def find_invalid(nums, preamble=25):
    for i, n in enumerate(nums):
        if i < preamble:
            continue
        test_nums = nums[i - preamble:i]
        if n not in (a + b for a, b in permutations(test_nums, 2)):
            return n


n = find_invalid(numbers)
print(f"Part 1: {n}")


def find_range(number, nums):
    for i in range(len(nums)):
        for j in range(len(nums)):
            if sum(nums[i:j]) == number:
                return nums[i:j]


r = find_range(n, numbers)
print(f"Part 1: {min(r)+max(r)}")
