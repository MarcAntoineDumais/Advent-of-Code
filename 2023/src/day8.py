import math


def load_data():
    file = open("data/day8.txt", "r")
    data = file.read().splitlines()
    file.close()

    return data

def parse_data(data):
    order = data[0]

    map = {}
    for line in data[2:]:
        node = line[:3]
        left = line[7:10]
        right = line[12:15]
        map[node]=(left, right)
    return order, map

def part1():
    data = load_data()
    order, map = parse_data(data)

    cur = "AAA"
    goal = "ZZZ"
    steps = 0
    i = 0
    while cur != goal:
        if order[i] == "L":
            cur = map[cur][0]
        else:
            cur = map[cur][1]
        i = (i + 1) % len(order)
        steps += 1

    print(f"{steps}")

def part2():
    data = load_data()
    order, map = parse_data(data)
    
    start_nodes = [x for x in map.keys() if x[2] == "A"]
    end_nodes = [x for x in map.keys() if x[2] == "Z"]
    
    pattern_map = {} # k=node v=(node after 1 cycle, cycles before end node, end node)
    for k in map.keys():
        node = k
        for c in order:
            node = map[node][0] if c == "L" else map[node][1]
        pattern_map[k] = [node, None, None]
        
    todo = start_nodes + end_nodes
    while len(todo) > 0:
        k = todo.pop()
        cur = k
        cycles = 0
        end_node = pattern_map[cur][0]
        while cur[2] != "Z" or (cur == k and cycles == 0):
            cycles += 1
            cur = end_node
            end_node = pattern_map[cur][0]
            
        pattern_map[k][1] = cycles
        pattern_map[k][2] = cur
        if pattern_map[cur][1] is None:
            todo += [cur]

    cycle_sizes = [v[1] for (k, v) in pattern_map.items() if k in start_nodes]
    cycles = math.lcm(*cycle_sizes)

    print(cycles * len(order))
