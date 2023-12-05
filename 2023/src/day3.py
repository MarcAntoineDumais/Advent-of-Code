def load_data():
    file = open("data/day3.txt", "r")
    data = file.read().splitlines()
    file.close()

    return data

def part1():
    data = load_data()

    sum = 0
    for y in range(len(data)):
        x = 0
        while x < len(data[y]):
            is_part, size = is_part_number(x, y, data)
            if is_part:
                number = int(data[y][x:x+size])
                sum += number
            x += size

    print(sum)



def part2():
    data = load_data()

    sum = 0
    for y in range(len(data)):
        for x in range(len(data[y])):
            if data[y][x] == "*":
                sum += adjacent_product(x, y, data)

    print(sum)

def is_part_number(x, y, data) -> (bool, int):
    if not data[y][x].isdigit():
        return False, 1
    
    size = 1
    for i in range(x + 1, len(data[y])):
        if not data[y][i].isdigit():
            break
        size += 1

    if x > 0 and is_symbol(data[y][x-1]):
        return True, size
    if x + size < len(data[y]) and is_symbol(data[y][x+size]):
        return True, size
    if y > 0:
        for i in range(max(0, x - 1), min(x + size + 1, len(data[y-1]))):
            if is_symbol(data[y-1][i]):
                return True, size
    if y < len(data) - 1:
        for i in range(max(0, x - 1), min(x + size + 1, len(data[y+1]))):
            if is_symbol(data[y+1][i]):
                return True, size

    return False, size

def is_symbol(c):
    return  not c.isdigit() and c != "."

def adjacent_product(x, y, data):
    product = 1
    count = 0
    
    if x > 0 and data[y][x-1].isdigit():
        n, _ = full_number(x-1, y, data)
        product *= n
        count += 1
    if x < len(data[y]) and data[y][x+1].isdigit():
        n, _ = full_number(x+1, y, data)
        product *= n
        count += 1
    
    i = x - 1
    while y > 0 and i < min(x + 2, len(data[y-1])):
        if data[y-1][i].isdigit():
            n, i = full_number(i, y-1, data)
            product *= n
            count += 1
        else:
            i += 1
    i = x - 1
    while y < len(data) - 1 and i < min(x + 2, len(data[y+1])):
        if data[y+1][i].isdigit():
            n, i = full_number(i, y+1, data)
            product *= n
            count += 1
        else:
            i += 1


    if count == 2:
        return product
    return 0

def full_number(x, y, data):
    while x > 0 and data[y][x-1].isdigit():
        x -= 1
    
    end = x + 1
    while end < len(data[y]) and data[y][end].isdigit():
        end += 1

    return int(data[y][x:end]), end