use grid::{Letter, COLS, NUMBERS, ROWS};
use std::{env, fs, path::Path};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("doggo.rs");
    let output = if let Some((c, dir)) = find() {
        format!("Found DOGGO in Cell({}, {}), {}", c.0, c.1, dir.str())
    } else {
        "Could not find DOGGO in the puzzle".to_owned()
    };
    fs::write(
        dest_path,
        format!(
            "\
#[inline]
fn doggo() {{
    unsafe {{
        printf(\"{}\\n\\0\".as_ptr().cast::<i8>());
    }}
}}
",
            output
        ),
    )
    .unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}

const fn get(Cell(row, col): Cell) -> Option<Letter> {
    if row >= 0 && col >= 0 && row < ROWS && col < COLS {
        // row and col are guaranteed to be positive because of the check above
        #[allow(clippy::cast_sign_loss)]
        Some(NUMBERS[((row * COLS) as usize) + col as usize])
    } else {
        None
    }
}

#[derive(Clone, Copy)]
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

impl Direction {
    const fn str(self) -> &'static str {
        match self {
            Self::Up => "Up",
            Self::Down => "Down",
            Self::Left => "Left",
            Self::Right => "Right",
            Self::UpLeft => "UpLeft",
            Self::UpRight => "UpRight",
            Self::DownLeft => "DownLeft",
            Self::DownRight => "DownRight",
        }
    }
}

#[derive(Clone, Copy)]
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

#[rustfmt::skip]
mod grid {
    #[derive(Copy, Clone)]
    pub enum Letter { D, O, G }

    use Letter::{D, O, G};

    pub const COLS: i8 = 14;
    pub const ROWS: i8 = 7;
    // It's col 7, row 2 DownRight
    pub const NUMBERS: [Letter; (COLS * ROWS) as usize] = [
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
