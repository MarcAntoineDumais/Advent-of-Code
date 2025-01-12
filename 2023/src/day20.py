import math

def load_data():
    with open("data/day20.txt", "r") as file:
        data = file.read().splitlines()

    modules = {}
    for line in data:
        parts = line.split(" -> ")
        module_type = line[0]
        name = parts[0]
        if module_type != "b":
            name = name[1:]
        destinations = parts[1].split(", ")

        modules[name] = (module_type, destinations)

    to_add = set()
    for _, module in modules.items():
        for destination in module[1]:
            if destination not in modules:
                to_add.add(destination)
    for destination in to_add:
        modules[destination] = (None, [])
    return modules

def part1():
    modules = load_data()
    
    flip_flop_memory, conjunction_memory = get_memories(modules)
        
    total_low_pulses = 0
    total_high_pulses = 0
    for _ in range(1000):
        low_pulses, high_pulses = pulse(modules, flip_flop_memory, conjunction_memory, ("button", "broadcaster", 0)) # from, to, value
        total_low_pulses += low_pulses
        total_high_pulses += high_pulses
        
    print(f"{total_low_pulses * total_high_pulses}")

def get_memories(modules):
    flip_flop_memory = set()
    conjunction_memory = {}
    for name, module in modules.items():
        if module[0] == "&":
            conjunction_memory[name] = {}
    
    for name, module in modules.items():
        for destination in module[1]:
            if destination in conjunction_memory:
                conjunction_memory[destination][name] = 0
    return flip_flop_memory, conjunction_memory

def pulse(modules, flip_flop_memory, conjunction_memory, pulse_info):
    low_pulses = 0
    high_pulses = 0

    pulses_todo = [pulse_info] 
    while len(pulses_todo) > 0:
        cur_pulse = pulses_todo.pop(0)
        mod = modules[cur_pulse[1]]

        if cur_pulse[2] == 0:
            low_pulses += 1
        else:
            high_pulses += 1

        if mod[0] is None:
            continue

        if mod[0] == "b":
            pulses_todo += [(cur_pulse[1], destination, cur_pulse[2]) for destination in mod[1]]

        elif mod[0] == "%":
            if cur_pulse[2] == 1:
                continue
            state = 0 if cur_pulse[1] not in flip_flop_memory else 1
            state = 1 - state
            if state == 0:
                flip_flop_memory.remove(cur_pulse[1])
            else:
                flip_flop_memory.add(cur_pulse[1])
            pulses_todo += [(cur_pulse[1], destination, state) for destination in mod[1]]

        else:
            conjunction_memory[cur_pulse[1]][cur_pulse[0]] = cur_pulse[2]
            send_low = True
            for _, v in conjunction_memory[cur_pulse[1]].items():
                if v == 0:
                    send_low = False
                    break
            pulses_todo += [(cur_pulse[1], destination, 0 if send_low else 1) for destination in mod[1]]
    
    return low_pulses, high_pulses

def part2():
    ## Never finishes
    # modules = load_data()
    # flip_flop_memory, conjunction_memory = get_memories(modules)
    # 
    # subgraphs = [("gb", "bt"), ("rb", "fv"), ("gn", "pr"), ("df", "rd")]
    # cycle_sizes = []
    # for subgraph in subgraphs:
    #     i = 0
    #     while True:
    #         i += 1
    #         pulse(modules, flip_flop_memory, conjunction_memory, ("broadcaster", subgraph[0], 0))
    #         if conjunction_memory["vd"][subgraph[1]] > 0:
    #             cycle_sizes += [i]
    #             print(f"cycle for {subgraph[0]}: {i}")
    #             break

    # Cycle sizes determined through graph visualization
    cycle_sizes = [3917, 3793, 3911, 3929]
    cycles = math.lcm(*cycle_sizes)
    print(f"{cycles}")

def export_to_graph():
    modules = load_data()

    conjunctions = {name for name, mod in modules.items() if mod[0] == "&"}
    flip_flops = {name for name, mod in modules.items() if mod[0] == "%"}
    get_name = lambda x: "&" + x if x in conjunctions else "\%" + x if x in flip_flops else x

    connections = []
    for name, mod in modules.items():
        origin = get_name(name)
        for destination in mod[1]:
            destination = get_name(destination)
            connections += [(origin, destination)]

    for conn in connections:
        print(f"\"{conn[0]}\" -> \"{conn[1]}\"")