def load_data():
    file = open("data/day2.txt", "r")
    data = file.readlines()
    file.close()

    return data

def part1():
    data = load_data()

    max = {"red": 12, "green": 13, "blue": 14}

    sum = 0
    for (i, line) in enumerate(data):
        parsed = parse(line)
        #print(f"max: {parsed}  line: {line}")
        if parsed["red"] <= max["red"] and parsed["green"] <= max["green"] and parsed["blue"] <= max["blue"]:
            sum += i + 1
    print(sum)



def part2():
    data = load_data()

    sum = 0
    for (i, line) in enumerate(data):
        parsed = parse(line)
        sum += parsed["red"] * parsed["green"] * parsed["blue"]
    print(sum)    

def parse(line):
    line = line.strip()
    max = {"red": 0, "green": 0, "blue": 0}
    line = line.split(": ")[1]
    sets = line.split("; ")
    for set in sets:
        counts = set.split(", ")
        for count in counts:
            tokens = count.split(" ")
            val = int(tokens[0])
            color = tokens[1]
            if color == "red" and val > max["red"]:
                max["red"] = val
            elif color == "green" and val > max["green"]:
                max["green"] = val
            elif color == "blue" and val > max["blue"]:
                max["blue"] = val
    return max