#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point2D(pub i32, pub i32);

impl Point2D {
    pub fn add(&self, other: &Point2D) -> Point2D {
        Point2D(self.0 + other.0, self.1 + other.1)
    }

    pub fn sub(&self, other: &Point2D) -> Point2D {
        Point2D(self.0 - other.0, self.1 - other.1)
    }

    pub fn magnitude_squared(&self) -> i32 {
        self.0 * self.0 + self.1 * self.1
    }

    pub fn clamp(&self, min: i32, max: i32) -> Point2D {
        Point2D(self.0.clamp(min, max), self.1.clamp(min, max))
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point2Di64(pub i64, pub i64);

impl Point2Di64 {
    pub fn add(&self, other: &Point2Di64) -> Point2Di64 {
        Point2Di64(self.0 + other.0, self.1 + other.1)
    }

    pub fn sub(&self, other: &Point2Di64) -> Point2Di64 {
        Point2Di64(self.0 - other.0, self.1 - other.1)
    }

    pub fn magnitude_squared(&self) -> i64 {
        self.0 * self.0 + self.1 * self.1
    }

    pub fn clamp(&self, min: i64, max: i64) -> Point2Di64 {
        Point2Di64(self.0.clamp(min, max), self.1.clamp(min, max))
    }
}

pub struct Intervals {
    pub ranges: Vec<Point2D>,
}

impl Intervals {
    pub fn new() -> Intervals {
        Intervals { ranges: vec![] }
    }

    pub fn add(&mut self, new_range: Point2D) {
        if new_range.1 <= new_range.0 {
            return;
        }
        if self.ranges.is_empty() {
            self.ranges.push(new_range);
            return;
        }

        let mut expand_range = None;
        for i in 0..self.ranges.len() {
            let range = &mut self.ranges[i];
            if new_range.0 < range.0 {
                if new_range.1 < range.0 {
                    self.ranges.insert(i, new_range);
                    return;
                }
                if new_range.1 <= range.1 {
                    range.0 = new_range.0;
                    return;
                }
                // new range ends after current range
                range.0 = new_range.0;
            } else if new_range.0 > range.1 {
                continue;
            }

            if new_range.1 <= range.1 {
                return;
            }

            range.1 = new_range.1;
            expand_range = Some(i);
            break;
        }

        if let Some(i) = expand_range {
            let range = self.ranges[i];

            let j = i + 1;
            while j < self.ranges.len() {
                let next_range = self.ranges[j];
                if range.1 < next_range.0 {
                    return;
                }
                self.ranges.remove(j);
                if range.1 <= next_range.1 {
                    self.ranges[i].1 = next_range.1;
                    return;
                }
            }
            return;
        }

        //new range is at the end
        self.ranges.push(new_range);
    }

    pub fn sub(&mut self, new_range: Point2D) {
        if new_range.1 <= new_range.0 {
            return;
        }
        let mut i = 0;
        while i < self.ranges.len() {
            let range = self.ranges[i];
            if new_range.1 <= range.0 {
                return;
            }
            if new_range.0 >= range.1 {
                i += 1;
                continue;
            }
            if new_range.0 > range.0 && new_range.1 < range.1 {
                let range_split = Point2D(new_range.1, range.1);
                self.ranges[i].1 = new_range.0;
                self.ranges.insert(i + 1, range_split);
                return;
            }
            if new_range.0 <= range.0 && new_range.1 >= range.1 {
                self.ranges.remove(i);
                continue;
            }
            if new_range.0 <= range.0 {
                self.ranges[i].0 = new_range.1;
                return;
            }
            if new_range.0 < range.1 {
                self.ranges[i].1 = new_range.0;
                i += 1;
                continue;
            }
        }
    }

    pub fn contains(&self, value: i32) -> bool {
        if self.ranges.is_empty() {
            return false;
        }
        for range in &self.ranges {
            if value < range.0 {
                return false;
            }
            if value >= range.1 {
                continue;
            }
            return true;
        }
        false
    }
}

pub fn get_2d_index(row: usize, col: usize, width: usize) -> usize {
    row * width + col
}

pub fn get_coords_from_index(i: usize, width: usize) -> (usize, usize) {
    (i / width, i % width)
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Orientation {
    Right,
    Up,
    Left,
    Down,
}

impl Orientation {
    pub fn rotate_cw(&self) -> Orientation {
        match self {
            Orientation::Right => Orientation::Down,
            Orientation::Up => Orientation::Right,
            Orientation::Left => Orientation::Up,
            Orientation::Down => Orientation::Left,
        }
    }

    pub fn rotate_ccw(&self) -> Orientation {
        match self {
            Orientation::Right => Orientation::Up,
            Orientation::Up => Orientation::Left,
            Orientation::Left => Orientation::Down,
            Orientation::Down => Orientation::Right,
        }
    }

    pub fn flip(&self) -> Orientation {
        match self {
            Orientation::Right => Orientation::Left,
            Orientation::Up => Orientation::Down,
            Orientation::Left => Orientation::Right,
            Orientation::Down => Orientation::Up,
        }
    }
}
