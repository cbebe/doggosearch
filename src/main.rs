use core::ops::{Add, Mul};

use grid::*;

fn get(Cell(row, col): Cell) -> Option<Letter> {
    if row < 0 || col < 0 || row >= ROWS || col >= COLS {
        None
    } else {
        NUMBERS.get(((row * COLS) as usize) + col as usize).copied()
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

#[derive(Clone, Copy, Debug)]
struct Cell(i32, i32);

impl Mul<i32> for Cell {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Add for Cell {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Clone, Copy)]
struct Line(Cell, Direction);

static NEIGHBORS: [Line; 8] = {
    use Direction::*;
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

fn main() {
    for row in 0..ROWS {
        for col in 0..COLS {
            let c = Cell(row, col);
            use Letter::*;
            for line in NEIGHBORS {
                let Line(n, dir) = line;
                if let (Some(D), Some(O), Some(G), Some(G), Some(O)) = (
                    get(c + (n * 0)),
                    get(c + (n * 1)),
                    get(c + (n * 2)),
                    get(c + (n * 3)),
                    get(c + (n * 4)),
                ) {
                    println!("Found DOGGO in {:?}, {:?}", c, dir);
                    return;
                }
            }
        }
    }
}

#[rustfmt::skip]
mod grid {
    use Letter::*;

    #[derive(Copy, Clone)]
    pub enum Letter { D, O, G }

    pub const COLS: i32 = 14;
    pub const ROWS: i32 = 7;
    // It's col 7, row 2 DownRight
    pub static NUMBERS: [Letter; (COLS * ROWS) as usize] = [
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
