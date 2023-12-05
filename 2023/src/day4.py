def load_data():
    file = open("data/day4.txt", "r")
    data = file.read().splitlines()
    file.close()

    return data

def part1():
    data = load_data()

    sum = 0
    for line in data:
        _, winning_numbers, our_numbers = parse_line(line)
        wins = count_wins(winning_numbers, our_numbers)
        if wins > 0:
            sum += 2 ** (wins - 1)

    print(sum)

def part2():
    data = load_data()

    sum = 0
    card_copies = {}
    for line in data:
        id, winning_numbers, our_numbers = parse_line(line)
        copies = 1 if id not in card_copies else card_copies[id] + 1
        sum += copies

        wins = count_wins(winning_numbers, our_numbers)
        
        for i in range(1, wins + 1):
            card_copies[id + i] = copies if (id + i) not in card_copies else card_copies[id + i] + copies
        
        if id in card_copies:
            del card_copies[id]

        #print(f"id: {id} copies: {copies} wins: {wins}")

    print(sum)

def parse_line(line):
    parts = line.split(":")
    id = int(parts[0].split()[1])
    parts = parts[1].split("|")

    winning_numbers = [int(x) for x in parts[0].split()]
    our_numbers = [int(x) for x in parts[1].split()]
    return id, winning_numbers, our_numbers

def count_wins(winning_numbers, our_numbers):
    winning_set = set(winning_numbers)
    wins = 0
    for n in our_numbers:
        if n in winning_set:
            wins += 1
    return wins