use crate::grid::{Letter, COLS, PUZZLE, ROWS};

#[must_use]
pub fn get_message() -> String {
    if let Some((c, dir)) = find() {
        format!("Found DOGGO in {c:?}, {dir:?}")
    } else {
        "Could not find DOGGO in the puzzle".to_owned()
    }
}

// why while loops are allowed in const fns and not for loops is beyond me
const fn find() -> Option<(Cell, Direction)> {
    let mut row = 0;
    while row < ROWS {
        let mut col = 0;
        while col < COLS {
            let c = Cell(row, col);
            let mut idx = 0;
            while idx < NEIGHBORS.len() {
                let line = NEIGHBORS[idx];
                if let Some(dir) = eval(c, line) {
                    return Some((c, dir));
                }
                idx += 1;
            }
            col += 1;
        }
        row += 1;
    }
    None
}

const fn get(Cell(row, col): Cell) -> Option<Letter> {
    if row >= 0 && col >= 0 && row < ROWS && col < COLS {
        // row and col are guaranteed to be positive because of the check above
        #[allow(clippy::cast_sign_loss)]
        Some(PUZZLE[((row * COLS) as usize) + col as usize])
    } else {
        None
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

#[derive(Debug, Clone, Copy)]
struct Cell(i8, i8);

// const traits are still experimental at the time of writing
impl Cell {
    const fn mul_scalar(self, rhs: i8) -> Self {
        Self(self.0 * rhs, self.1 * rhs)
    }

    const fn add_cell(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Clone, Copy)]
struct Line(Cell, Direction);

const NEIGHBORS: [Line; 8] = {
    use Direction::{Down, DownLeft, DownRight, Left, Right, Up, UpLeft, UpRight};
    [
        Line(Cell(-1, 0), Up),
        Line(Cell(1, 0), Down),
        Line(Cell(0, -1), Left),
        Line(Cell(0, 1), Right),
        Line(Cell(-1, -1), UpLeft),
        Line(Cell(-1, 1), UpRight),
        Line(Cell(1, -1), DownLeft),
        Line(Cell(1, 1), DownRight),
    ]
};

const fn eval(c: Cell, line: Line) -> Option<Direction> {
    let Line(n, dir) = line;
    if let (Some(Letter::D), Some(Letter::O), Some(Letter::G), Some(Letter::G), Some(Letter::O)) = (
        get(c),
        get(c.add_cell(n)),
        get(c.add_cell(n.mul_scalar(2))),
        get(c.add_cell(n.mul_scalar(3))),
        get(c.add_cell(n.mul_scalar(4))),
    ) {
        Some(dir)
    } else {
        None
    }
}

#[rustfmt::skip]
mod grid {
    #[derive(Copy, Clone)]
    pub enum Letter { D, O, G }

    use Letter::{D, O, G};

    pub const COLS: i8 = 14;
    pub const ROWS: i8 = 7;
    // It's col 7, row 2 DownRight
    pub const PUZZLE: [Letter; (COLS * ROWS) as usize] = [
    //                       v
    //  0  1  2  3  4  5  6  7  8  9  10 11 12 13
        D, G, O, O, D, D, O, D, G, O, O, D, D, O, // 0
        O, D, O, O, G, G, G, D, O, D, G, O, G, G, // 1
        O, G, O, G, D, O, O, D, G, O, O, D, D, D, // 2 <
        D, G, D, O, O, O, G, G, O, O, G, D, G, O, // 3
        O, G, D, G, O, G, D, G, O, G, G, O, G, D, // 4
        D, D, D, G, D, D, O, D, O, O, G, D, O, O, // 5
        O, D, G, O, G, G, D, O, O, G, G, O, O, D, // 6
    ];
}
