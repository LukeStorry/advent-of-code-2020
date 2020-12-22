from collections import deque
with open("../inputs/22.txt") as file:
    data = file.read().strip()

deck_1, deck_2 = (deque(int(x) for x in player.splitlines()[1:]) for player in data.split("\n\n"))

round = 1
while deck_1 and deck_2:
    round += 1
    card_1, card_2 = deck_1.popleft(), deck_2.popleft()
    if card_1 > card_2:
        deck_1.extend((card_1, card_2))
    else:
        deck_2.extend((card_2, card_1))

score = sum(x * y for x, y in enumerate(reversed(deck_1 if deck_1 else deck_2), 1))
print(score)
