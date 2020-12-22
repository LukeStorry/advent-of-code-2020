from collections import deque


def play(deck_1, deck_2) -> int:
    while deck_1 and deck_2:
        card_1, card_2 = deck_1.popleft(), deck_2.popleft()
        if card_1 > card_2:
            deck_1.extend((card_1, card_2))
        else:
            deck_2.extend((card_2, card_1))
    return 1 if deck_1 else 2


def play_recursive(deck_1, deck_2):
    previous_decks = []
    while deck_1 and deck_2:
        state = ','.join(str(int(i)) for i in deck_1) + "/" + ','.join(str(int(i)) for i in deck_2)
        if state in previous_decks:
            return 1
        previous_decks.append(state)

        card_1, card_2 = deck_1.popleft(), deck_2.popleft()

        one_won = card_1 > card_2 if len(deck_1) < card_1 or len(deck_2) < card_2 \
            else play_recursive(deque(list(deck_1)[:card_1]), deque(list(deck_2)[:card_2])) == 1

        if one_won:
            deck_1.extend((card_1, card_2))
        else:
            deck_2.extend((card_2, card_1))

    return 1 if deck_1 else 2


def score(deck):
    return sum(x * y for x, y in enumerate(reversed(deck), 1))


with open("../inputs/22.txt") as file:
    data = file.read().strip()
decks = [[int(x) for x in player.splitlines()[1:]] for player in data.split("\n\n")]

deck_1, deck_2 = (deque(deck) for deck in decks)
winner = play(deck_1, deck_2)
print("Part 1: ", score(deck_1 if winner == 1 else deck_2))

deck_1, deck_2 = (deque(deck) for deck in decks)
winner = play_recursive(deck_1, deck_2)
print("Part 2: ", score(deck_1 if winner == 1 else deck_2))
