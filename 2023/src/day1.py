def load_data():
    file = open("data/day1.txt", "r")
    data = file.readlines()
    file.close()

    return data

def part1():
    data = load_data()
    
    sum = 0
    for line in data:
        first = None
        last = None
        for c in line:
            if c.isdigit():
                if first is None:
                    first = int(c)
                last = int(c)
        
        if first is None or last is None:
            print(f"No first or last digit in {line}")
            return
        sum += first * 10 + last
    print(sum)



def part2():
    data = load_data()

    sum = 0
    for line in data:
        first = None
        for i in range(len(line)):
            first = parse(line, i)
            if first is not None:
                break
        for i in reversed(range(len(line))):
            last = parse(line, i)
            if last is not None:
                break
        
        if first is None or last is None:
            print(f"No first or last digit in {line}")
            return
        sum += first * 10 + last
    print(sum)

def parse(line, i):
    if line[i].isdigit():
        return int(line[i])
    if line[i:i+3] == "one":
        return 1
    if line[i:i+3] == "two":
        return 2
    if line[i:i+5] == "three":
        return 3
    if line[i:i+4] == "four":
        return 4
    if line[i:i+4] == "five":
        return 5
    if line[i:i+3] == "six":
        return 6
    if line[i:i+5] == "seven":
        return 7
    if line[i:i+5] == "eight":
        return 8
    if line[i:i+4] == "nine":
        return 9
    return None