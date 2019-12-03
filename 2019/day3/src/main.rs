use log::{info, trace};
use std::collections::HashSet;
use std::fs;
use std::io::Write;

#[derive(Debug, Default, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn dist(&self) -> i64 {
        // (self.x + self.y).abs()
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug, Default)]
struct Maze {
    pub coords: HashSet<Point>,
    history: Vec<Point>,
    pub current: Point,
}

impl Maze {
    fn track(&mut self, i: Instruction) {
        match i {
            Instruction::Left(d) => {
                info!("left instruction, distance {}", d);
                for _ in 0..d {
                    trace!(
                        "going left one place; state: {:?}, seen {} points",
                        self.current,
                        self.coords.len()
                    );
                    self.current.x -= 1;
                    self.add();
                }
            }
            Instruction::Right(d) => {
                info!("right instruction, distance {}", d);
                for _ in 0..d {
                    trace!(
                        "going right one place; state: {:?}, seen {} points",
                        self.current,
                        self.coords.len()
                    );
                    self.current.x += 1;
                    self.add();
                }
            }
            Instruction::Up(d) => {
                info!("up instruction, distance {}", d);
                for _ in 0..d {
                    trace!(
                        "going up one place; state: {:?}, seen {} points",
                        self.current,
                        self.coords.len()
                    );
                    self.current.y += 1;
                    self.add();
                }
            }
            Instruction::Down(d) => {
                info!("down instruction, distance {}", d);
                for _ in 0..d {
                    trace!(
                        "going down one place; state: {:?}, seen {} points",
                        self.current,
                        self.coords.len()
                    );
                    self.current.y -= 1;
                    self.add();
                }
            }
        }
    }

    fn add(&mut self) {
        self.coords.insert(self.current);
        self.history.push(self.current);
    }

    fn intersections_with<'a>(&'a self, other: &'a Maze) -> Vec<&'a Point> {
        self.coords.intersection(&other.coords).collect()
    }

    fn write<P>(&self, fname: P)
    where
        P: AsRef<std::path::Path>,
    {
        let mut out = std::fs::File::create(fname).expect("opening output file");
        for point in &self.history {
            writeln!(&mut out, "{} {}", point.x, point.y).unwrap();
        }
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
        let ins = if s.starts_with("R") {
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
        };

        info!("parsing {} as {:?}", s, ins);
        ins
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

fn run_on_string(s: &str) -> i64 {
    let lines = s.lines().collect::<Vec<_>>();

    // Run the first line
    let mut l1_maze = Maze::default();
    for instruction in lines[0].split(",") {
        let instruction = instruction.trim().into();
        l1_maze.track(instruction);
    }

    l1_maze.write("l1_maze.txt");

    let mut l2_maze = Maze::default();
    for instruction in lines[1].split(",") {
        let instruction = instruction.trim().into();
        l2_maze.track(instruction);
    }

    l2_maze.write("l2_maze.txt");

    let intersections = l1_maze.intersections_with(&l2_maze);

    let closest_point = compute_closest_point(&intersections);
    closest_point.dist()
}

fn main() {
    let input_text = fs::read_to_string("input.txt").unwrap();
    println!("Closest distance: {}", run_on_string(&input_text));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_first_example() {
        init();

        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(run_on_string(input), 159);
    }

    #[test]
    fn test_second_example() {
        init();

        let input =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(run_on_string(input), 135);
    }
}
