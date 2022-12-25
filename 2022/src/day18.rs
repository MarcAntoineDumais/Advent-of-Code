use std::{collections::HashMap, fs};

pub fn run() {
    let data = fs::read_to_string("data/day18.txt").unwrap();
    let mut cubes: Vec<Cube> = data
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut split = line.split(",");
            Cube(
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let mut surface_area = calc_surface_area(&cubes);
    println!("{surface_area}");

    // Part 2
    let mut min_cube = Cube(i32::MAX, i32::MAX, i32::MAX);
    let mut max_cube = Cube(i32::MIN, i32::MIN, i32::MIN);
    for cube in cubes.iter() {
        min_cube = min_cube.min(cube);
        max_cube = max_cube.max(cube);
    }
    min_cube = min_cube.add(&Cube(-1, -1, -1));
    max_cube = max_cube.add(&Cube(1, 1, 1));

    let mut groups = HashMap::new();
    for cube in cubes.clone() {
        groups.insert(cube, 0);
    }

    let mut next_group = 1;
    for x in min_cube.0..=max_cube.0 {
        for y in min_cube.1..=max_cube.1 {
            for z in min_cube.2..=max_cube.2 {
                let cur_cube = Cube(x, y, z);
                if !cur_cube.within(&min_cube, &max_cube) {
                    continue;
                }

                if groups.contains_key(&cur_cube) {
                    continue;
                }

                let mut to_mark = vec![cur_cube];
                while !to_mark.is_empty() {
                    let cur_cube_2 = to_mark.pop().unwrap();
                    if !cur_cube_2.within(&min_cube, &max_cube) {
                        continue;
                    }
                    if groups.contains_key(&cur_cube_2) {
                        continue;
                    }
                    groups.insert(cur_cube_2.clone(), next_group);
                    for neighbor in cur_cube_2.neighbors() {
                        to_mark.push(neighbor);
                    }
                }

                next_group += 1;
            }
        }
    }

    for (cube, group) in groups {
        if group > 1 {
            cubes.push(cube);
        }
    }
    surface_area = calc_surface_area(&cubes);
    println!("{surface_area}");
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Cube(i32, i32, i32);

impl Cube {
    fn neighbors(&self) -> Vec<Cube> {
        vec![
            Cube(self.0 + 1, self.1, self.2),
            Cube(self.0 - 1, self.1, self.2),
            Cube(self.0, self.1 + 1, self.2),
            Cube(self.0, self.1 - 1, self.2),
            Cube(self.0, self.1, self.2 + 1),
            Cube(self.0, self.1, self.2 - 1),
        ]
    }

    fn min(&self, other: &Cube) -> Cube {
        Cube(
            self.0.min(other.0),
            self.1.min(other.1),
            self.2.min(other.2),
        )
    }

    fn max(&self, other: &Cube) -> Cube {
        Cube(
            self.0.max(other.0),
            self.1.max(other.1),
            self.2.max(other.2),
        )
    }

    fn add(&self, other: &Cube) -> Cube {
        Cube(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }

    fn within(&self, min_bound: &Cube, max_bound: &Cube) -> bool {
        self.0 >= min_bound.0
            && self.0 <= max_bound.0
            && self.1 >= min_bound.1
            && self.1 <= max_bound.1
            && self.2 >= min_bound.2
            && self.2 <= max_bound.2
    }
}

fn calc_surface_area(cubes: &Vec<Cube>) -> i32 {
    let mut surface_area = 0;
    let mut cubes_map: HashMap<Cube, bool> = HashMap::new();
    for cube in cubes.iter() {
        if cubes_map.contains_key(cube) {
            continue;
        }

        surface_area += 6;
        for neighbor in cube.neighbors() {
            if cubes_map.contains_key(&neighbor) {
                surface_area -= 2;
            }
        }
        cubes_map.insert(*cube, true);
    }
    surface_area
}
