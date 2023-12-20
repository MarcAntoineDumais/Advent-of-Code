def load_data():
    with open("data/day15.txt", "r") as file:
        data = file.read().strip().split(",")
    return data

def part1():
    steps = load_data()
    
    total_hash = sum([calc_hash(step) for step in steps])

    print(f"{total_hash}")

def calc_hash(step):
    hash = 0
    for c in step:
        hash = ((hash + ord(c)) * 17) % 256
    return hash

def part2():
    steps = load_data()
    
    # Parse lens instructions
    lens_steps = []
    for step in steps:
        parts = step.split("=")
        label = step[:-1]
        if step[-1] != "-":
            label = parts[0]

        hash = calc_hash(label)

        if step[-1] == "-":
            lens_steps += [(hash, "-", label)]
        else:
            lens_steps += [(hash, "=", label, int(parts[1]))]
    
    # Execute instructions
    boxes = []
    for _ in range(256):
        boxes += [[]]
    
    for step in lens_steps:
        if step[1] == "-":
            for i in range(len(boxes[step[0]])):
                if boxes[step[0]][i][0] == step[2]:
                    boxes[step[0]].pop(i)
                    break
        else:
            broken = False
            for i in range(len(boxes[step[0]])):
                if boxes[step[0]][i][0] == step[2]:
                    boxes[step[0]][i][1] = step[3]
                    broken = True
                    break
            if not broken:
                boxes[step[0]] += [[step[2], step[3]]]

    # Calculate focusing power
    total_focusing_power = 0
    for i, box in enumerate(boxes):
        for j, lens in enumerate(box):
            total_focusing_power += (1 + i) * (j + 1) * lens[1]

    print(f"{total_focusing_power}")