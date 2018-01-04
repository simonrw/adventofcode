type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

pub type Maze<'a> = &'a [&'a [char]];
pub type StaticMaze = Maze<'static>;

#[derive(Default, Eq, PartialEq, Debug, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

/*
fn main() {
}



type Label = char;

#[derive(Eq, PartialEq, Debug)]
pub struct Marker {
    position: Point,
    piece: Piece,
    seen_labels: Vec<Label>,
}

impl Marker {
    pub fn next(&self, maze: Maze) -> Option<Marker> {
        match self.piece {
            Piece::Horizontal => {
                let new_pos = Point { x: self.position.x + 1usize, y: self.position.y };
                None
            },
            Piece::Vertical => {
                let new_pos = Point { x: self.position.x, y: self.position.y };
                None
            },
            Piece::Cross => {
                None
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Piece {
    Horizontal,
    Vertical,
    Cross,
}

pub fn find_start_point(maze: Maze) -> Marker {
    let height = maze.len();
    let width = maze[0].len();

    for y in 0..height {
        let line = maze[y];
        assert_eq!(line.len(), width);
        for x in 0..width {
            let c = line[x];
            match c {
                '|' | '-' => {
                    if y == 0 || x == 0 || x == (width - 1usize) || y == (height - 1usize) {
                        return Marker {
                            position: Point { x: x, y: y },
                            piece: if c == '|' { Piece::Vertical } else { Piece::Horizontal },
                            seen_labels: Vec::new(),
                        }
                    }
                },
                _ => {},
            }
        }
    }

    unreachable!();
}
*/

fn validate_maze_dimensions(maze: StaticMaze, width: usize, height: usize) -> Result<()> {
    for i in 0..height {
        let line = maze[i];
        if line.len() != width {
            return Err("Dimension mismatch".into());
        }
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    NoDirection,
    North,
    South,
    East,
    West,
}

fn has_start<'a>(line: &'a [char]) -> bool {
    false
}

fn char_at_point(maze: StaticMaze, point: &Point) -> char {
    maze[point.y][point.x]
}

fn count_chars(maze: StaticMaze) -> Result<Vec<char>> {
    let mut out = Vec::new();
    let height = maze.len();
    let width = maze[0].len();

    validate_maze_dimensions(maze, width, height)?;

    /* Find the start point */
    let mut point = Point::default();
    let mut direction = Direction::NoDirection;

    /* Check the horizontal stretches first */
    for i in 0..width {
        let c = maze[0][i];
        if c != ' ' {
            if c != '|' {
                continue;
            }

            if i == 0 || i == (width - 1) {
                unreachable!("Have not handled this case :(");
            }

            point = Point { x: i, y: 0 };
            direction = Direction::South;
            break;
        }

        let c = maze[height - 1][i];
        if c != ' ' {
            if c != '|' {
                continue;
            }

            if i == 0 || i == (width - 1) {
                unreachable!("Have not handled this case :(");
            }

            point = Point { x: i, y: height - 1 };
            direction = Direction::North;
            break;
        }
    }

    /* Check the vertical stretches next */
    for i in 0..height {
        let c = maze[i][0];
        if c != ' ' {
            if c != '-' {
                continue;
            }

            if i == 0 || i == (height - 1) {
                unreachable!("Have not handled this case :(");
            }

            point = Point { x: 0, y: i };
            direction = Direction::East;
            break;
        }

        let c = maze[i][width - 1];
        if c != ' ' {
            if c != '-' {
                continue;
            }

            if i == 0 || i == (height - 1) {
                unreachable!("Have not handled this case :(");
            }

            point = Point { x: width - 1, y: i };
            direction = Direction::West;
            break;
        }
    }

    println!("Found start point {:?} facing {:?}", point, direction);

    loop {
        let next_point = match direction {
            Direction::North => Point { x: point.x, y: point.y - 1 },
            Direction::South => Point { x: point.x, y: point.y + 1 },
            Direction::East => Point { x: point.x + 1, y: point.y },
            Direction::West => Point { x: point.x - 1, y: point.y },
            _ => unreachable!(),
        };

        if next_point.x >= width {
            break;
        }

        if next_point.y >= height {
            break;
        }

        let next_char = char_at_point(maze, &next_point);
        match next_char {
            '|' | '-' | '+' => {},
            ' ' => panic!("Bad situation"),
            _ => out.push(next_char),
        }

        point = next_point;
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_MAZE: StaticMaze = &[
        &[' ',' ',' ',' ',' ','|',' ',' ',' ',' ',' ',' ',' ',' ',' '],
        &[' ',' ',' ',' ',' ','|',' ',' ','+','-','-','+',' ',' ',' '],
        &[' ',' ',' ',' ',' ','A',' ',' ','|',' ',' ','C',' ',' ',' '],
        &[' ','F','-','-','-','|','-','-','-','-','E','|','-','-','+'],
        &[' ',' ',' ',' ',' ','|',' ',' ','|',' ',' ','|',' ',' ','D'],
        &[' ',' ',' ',' ',' ','+','B','-','+',' ',' ','+','-','-','+'],
    ];

    /*
    #[test]
    fn test_find_start() {
        assert_eq!(find_start_point(EXAMPLE_MAZE).position, Point { x: 5, y: 0 });
    }

    #[test]
    fn test_problem() {
        let mut point = find_start_point(EXAMPLE_MAZE);
        loop {
            match point.next(EXAMPLE_MAZE) {
                Some(next) => { point = next; }
                None => break,
            }
        }
        assert_eq!(point.seen_labels, vec!['A', 'B', 'C', 'D', 'E', 'F']);
    }
    */

    #[test]
    fn test_main_function() {
        assert_eq!(count_chars(EXAMPLE_MAZE).unwrap(), vec!['A', 'B', 'C', 'D', 'E', 'F']);
    }
}
