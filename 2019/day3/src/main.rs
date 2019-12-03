use std::collections::HashSet;
use std::fs;

#[derive(Debug, Default, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn dist(&self) -> i64 {
        (self.x + self.y).abs()
    }
}

#[derive(Debug, Default)]
struct Maze {
    pub coords: HashSet<Point>,
    pub current: Point,
}

impl Maze {
    fn track(&mut self, i: Instruction) {
        match i {
            Instruction::Left(d) => {
                for _ in 0..d {
                    self.current.x -= 1;
                    self.coords.insert(self.current);
                }
            }
            Instruction::Right(d) => {
                for _ in 0..d {
                    self.current.x += 1;
                    self.coords.insert(self.current);
                }
            }
            Instruction::Up(d) => {
                for _ in 0..d {
                    self.current.y += 1;
                    self.coords.insert(self.current);
                }
            }
            Instruction::Down(d) => {
                for _ in 0..d {
                    self.current.y -= 1;
                    self.coords.insert(self.current);
                }
            }
        }
    }

    fn intersections_with<'a>(&'a self, other: &'a Maze) -> Vec<&'a Point> {
        self.coords.intersection(&other.coords).collect()
    }
}

#[derive(Debug)]
enum Instruction {
    Left(i64),
    Right(i64),
    Up(i64),
    Down(i64),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Instruction {
        if s.starts_with("R") {
            let (_, dist_text) = s.split_at(1);
            Instruction::Right(dist_text.parse().unwrap())
        } else if s.starts_with("L") {
            let (_, dist_text) = s.split_at(1);
            Instruction::Left(dist_text.parse().unwrap())
        } else if s.starts_with("U") {
            let (_, dist_text) = s.split_at(1);
            Instruction::Up(dist_text.parse().unwrap())
        } else if s.starts_with("D") {
            let (_, dist_text) = s.split_at(1);
            Instruction::Down(dist_text.parse().unwrap())
        } else {
            unreachable!()
        }
    }
}

fn compute_closest_point<'a>(points: &'a [&'a Point]) -> &'a Point {
    let mut dist = points[0].dist();
    let mut closest_point = points[0];

    for point in points {
        let current_dist = point.dist();
        if current_dist < dist {
            dist = current_dist;
            closest_point = point;
        }
    }

    closest_point
}

fn main() {
    let input_text = fs::read_to_string("input.txt").unwrap();
    let lines = input_text.lines().collect::<Vec<_>>();

    // Run the first line
    let mut l1_maze = Maze::default();
    for instruction in lines[0].split(",") {
        let instruction = instruction.trim().into();
        l1_maze.track(instruction);
    }

    let mut l2_maze = Maze::default();
    for instruction in lines[1].split(",") {
        let instruction = instruction.trim().into();
        l2_maze.track(instruction);
    }

    let intersections = l1_maze.intersections_with(&l2_maze);

    let closest_point = compute_closest_point(&intersections);
    println!("Closest point: {:?}", closest_point);
    println!("Distance: {}", closest_point.dist());
}

#[cfg(test)]
mod tests {}
