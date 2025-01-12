def load_data():
    with open("data/day19.txt", "r") as file:
        data = file.read().splitlines()

    workflows = {}
    i = 0
    while data[i] != "":
        name, rules = parse_workflow(data[i])
        workflows[name] = rules
        i += 1

    parts = [parse_part(line) for line in data[i+1:]]
    return workflows, parts

def parse_workflow(line):
    parts = line.split("{")
    name = parts[0]
    unparsed_rules = parts[1][:-1].split(",")
    last_rule = unparsed_rules[-1]
    rules = []
    for r in unparsed_rules[:-1]:
        parts = r.split(":")
        rules += [(parts[0][0], parts[0][1], int(parts[0][2:]), parts[1])]

    return name, (rules, last_rule)

def parse_part(line):
    line = line[1:-1]
    parts = line.split(",")
    vals = {}
    for part in parts:
        parts2 = part.split("=")
        vals[parts2[0]] = int(parts2[1])
    return vals

def part1():
    workflows, parts = load_data()
    
    total = 0
    for part in parts:
        cur = "in"
        while cur not in ["A", "R"]:
            workflow = workflows[cur]
            broken = False
            for rule in workflow[0]:
                if rule[1] == "<":
                    if part[rule[0]] < rule[2]:
                        cur = rule[3]
                        broken = True
                        break
                elif part[rule[0]] > rule[2]:
                    cur = rule[3]
                    broken = True
                    break

            
            if broken:
                continue
            cur = workflow[1]
        
        if cur == "A":
            total += part["x"] + part["m"] + part["a"] + part["s"]

    print(total)

def part2():
    workflows, _ = load_data()
    
    ranges = ((1, 4001), (1, 4001), (1, 4001), (1, 4001))
    combinations = solve(ranges, workflows, "in")
    print(combinations)

attributeMap = {"x": 0, "m": 1, "a": 2, "s": 3}

def solve(ranges, workflows, current_workflow):
    for r in ranges:
        if r[1] <= r[0]:
            return 0
    if current_workflow == "A":
        return (ranges[0][1] - ranges[0][0]) * (ranges[1][1] - ranges[1][0]) * (ranges[2][1] - ranges[2][0]) * (ranges[3][1] - ranges[3][0])
    elif current_workflow == "R":
        return 0

    workflow = workflows[current_workflow]

    combinations = 0
    cur_ranges = list(ranges)
    for rule in workflow[0]:
        cur_range = cur_ranges[attributeMap[rule[0]]]
        if rule[1] == "<":
            if cur_range[1] <= rule[2]:
                return solve(tuple(cur_ranges), workflows, rule[3])
            elif cur_range[0] >= rule[2]:
                pass # current rule doesn't apply
            else:
                inf_range = (cur_range[0], rule[2])
                sup_range = (rule[2], cur_range[1])

                tmp_ranges = cur_ranges.copy()
                tmp_ranges[attributeMap[rule[0]]] = inf_range
                combinations += solve(tuple(tmp_ranges), workflows, rule[3])
                
                cur_ranges[attributeMap[rule[0]]] = sup_range
        else:
            if cur_range[0] > rule[2]:
                return solve(tuple(cur_ranges), workflows, rule[3])
            elif cur_range[1] <= rule[2]:
                pass # current rule doesn't apply
            else:
                inf_range = (cur_range[0], rule[2] + 1)
                sup_range = (rule[2] + 1, cur_range[1])

                tmp_ranges = cur_ranges.copy()
                tmp_ranges[attributeMap[rule[0]]] = sup_range
                combinations += solve(tuple(tmp_ranges), workflows, rule[3])
                
                cur_ranges[attributeMap[rule[0]]] = inf_range
    
    return combinations + solve(tuple(cur_ranges), workflows, workflow[1])
