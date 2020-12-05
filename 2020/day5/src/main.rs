fn main() {
    tracing_subscriber::fmt::init();
    let input = include_str!("../input");
    let layout = Layout::default();
    let answer = input
        .lines()
        .map(|l| l.trim())
        .map(|l| layout.seat_position(l).seat_id())
        .max()
        .unwrap_or(0);
    println!("{}", answer);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    row: u64,
    column: u64,
}

impl Position {
    fn seat_id(&self) -> u64 {
        self.row * 8 + self.column
    }
}

#[derive(Debug)]
enum Instruction {
    Front,
    Back,
    Left,
    Right,
}

struct Layout {
    n_rows: u64,
    n_columns: u64,
}

impl Layout {
    #[tracing::instrument]
    fn new(n_rows: u64, n_columns: u64) -> Self {
        Self { n_rows, n_columns }
    }

    #[tracing::instrument(skip(self))]
    fn seat_position(&self, input: &str) -> Position {
        let instructions = self.parse_input(input);
        let mut row_range = (0, self.n_rows - 1);
        let mut col_range = (0, self.n_columns - 1);

        for instruction in &instructions {
            let span = tracing::debug_span!("instruction", ?instruction, ?col_range, ?row_range);
            let _enter = span.enter();
            match instruction {
                Instruction::Front => {
                    row_range.1 = (row_range.0 + row_range.1) / 2;
                    tracing::debug!(after = ?row_range, "updating row range");
                }
                Instruction::Back => {
                    row_range.0 = (row_range.0 + row_range.1) / 2 + 1;
                    tracing::debug!(after = ?row_range, "updating row range");
                }
                Instruction::Left => {
                    col_range.1 = (col_range.0 + col_range.1) / 2;
                    tracing::debug!(after = ?col_range, "updating col range");
                }
                Instruction::Right => {
                    col_range.0 = (col_range.0 + col_range.1) / 2 + 1;
                    tracing::debug!(after = ?col_range, "updating col range");
                }
            }
        }

        tracing::debug!(row_range = ?row_range, col_range = ?col_range, "final result");
        assert_eq!(row_range.0, row_range.1);
        assert_eq!(col_range.0, col_range.1);

        Position {
            row: row_range.0,
            column: col_range.0,
        }
    }

    #[tracing::instrument(skip(self))]
    fn parse_input(&self, input: &str) -> Vec<Instruction> {
        input
            .chars()
            .map(|c| match c {
                'F' => Instruction::Front,
                'B' => Instruction::Back,
                'L' => Instruction::Left,
                'R' => Instruction::Right,
                _ => unreachable!("badly formed input: {}", c),
            })
            .collect()
    }
}

// Default for part 1
impl std::default::Default for Layout {
    fn default() -> Self {
        Self::new(128, 8)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        init();
        run_example("BFFFBBFRRR", Position { row: 70, column: 7 });
    }

    #[test]
    fn example_2() {
        init();
        run_example("FFFBBBFRRR", Position { row: 14, column: 7 });
    }

    #[test]
    fn example_3() {
        init();
        run_example(
            "BBFFBBFRLL",
            Position {
                row: 102,
                column: 4,
            },
        );
    }

    #[test]
    fn worked_example() {
        init();
        run_example("FBFBBFFRLR", Position { row: 44, column: 5 });
    }

    fn init() {
        let _ = tracing_subscriber::fmt::try_init();
    }

    fn run_example(input: &str, expected: Position) {
        let layout = Layout::default();
        assert_eq!(layout.seat_position(input), expected);
    }
}
