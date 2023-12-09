def load_data():
    file = open("data/day6.txt", "r")
    data = file.read().splitlines()
    file.close()

    times = [int(x) for x in data[0].split(":")[1].split()]
    distances = [int(x) for x in data[1].split(":")[1].split()]

    return times, distances

def part1():
    times, distances = load_data()
    
    total_ways = 1
    for race in range(len(times)):
        total_ways *= count_ways(times[race], distances[race])

    print(total_ways)

def part2():
    times, distances = load_data()
    total_time = int("".join([str(x) for x in times]))
    total_distance = int("".join([str(x) for x in distances]))

    print(count_ways(total_time, total_distance))
    
def count_ways(time, record):
    ways = 0
    for i in range(time):
        if i * (time - i) > record:
            ways += 1
    return ways