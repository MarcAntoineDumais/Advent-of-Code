def load_data():
    file = open("data/day7.txt", "r")
    data = file.read().splitlines()
    file.close()

    return data

def part1():
    data = load_data()
    
    hands = [(get_card_strength(line[:5]), get_rank(line[:5]), int(line[6:])) for line in data]
    hands = sorted(hands, key = lambda x: (x[1], x[0][0], x[0][1], x[0][2], x[0][3], x[0][4]))

    winnings = sum([(i + 1) * x[2] for (i, x) in enumerate(hands)])
    print(f"{winnings}")

def get_rank(hand, is_part_2: False):
    highest_count = 0
    char_counts = {}
    jokers = 0
    for c in hand:
        if c in char_counts:
            char_counts[c] += 1
        else:
            char_counts[c] = 1
        highest_count = max(highest_count, char_counts[c])
        if c == "J":
            jokers += 1

    pairs = sum([1 if v==2 else 0 for (_, v) in char_counts.items()])

    if highest_count == 5:
        return 6
    if highest_count == 4:
        if is_part_2 and jokers > 0:
            return 6
        return 5
    if highest_count == 3:
        if is_part_2:
            if jokers == 2 or (jokers == 3 and pairs == 1):
                return 6
            if jokers == 3 or jokers == 1:
                return 5
        for c in hand:
            if char_counts[c] == 2:
                return 4
        if is_part_2 and jokers > 0:
            return 4
        return 3
    if highest_count == 2:
        if is_part_2:
            if jokers == 2:
                if pairs == 1:
                    return 3
                if pairs == 2:
                    return 5
            if jokers == 1:
                if pairs == 1:
                    return 3
                if pairs == 2:
                    return 4
        return pairs
    if is_part_2 and jokers == 1:
        return 1
    return 0
    
def get_card_strength(hand, is_part_2: False):
    map = {"2": 0, "3": 1, "4": 2, "5": 3, "6": 4, "7": 5, "8": 6, "9": 7, "T": 8, "J": 9, "Q": 10,"K": 11, "A": 12}
    if is_part_2:
        map = {"2": 1, "3": 2, "4": 3, "5": 4, "6": 5, "7": 6, "8": 7, "9": 8, "T": 9, "J": 0, "Q": 10,"K": 11, "A": 12}
    return [map[c] for c in hand]

def part2():
    data = load_data()
    
    hands = [(get_card_strength(line[:5], True), get_rank(line[:5], True), int(line[6:])) for line in data]
    hands = sorted(hands, key = lambda x: (x[1], x[0][0], x[0][1], x[0][2], x[0][3], x[0][4]))

    winnings = sum([(i + 1) * x[2] for (i, x) in enumerate(hands)])
    print(f"{winnings}")
        