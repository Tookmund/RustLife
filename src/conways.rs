extern crate multiarray;

use multiarray::Array2D;
use std::usize::MAX;

type Board = Array2D<u8>;
type RC = usize;
type Rule = std::vec::Vec<RC>;

pub struct Life {
    rows: RC,
    cols: RC,
    board: Board,
    wraparound: bool,
    birth: Rule,
    survive: Rule,
}

impl Life {
    pub fn new(rows: RC, cols: RC) -> Self {
        Self {
            rows: rows,
            cols: cols,
            board: Array2D::new([rows, cols], 0),
            wraparound: true,
            birth: vec![3],
            survive: vec![2, 3],
        }
    }
    fn kill(&mut self, row: RC, column: RC) {
        self.board[[row,column]] = 0;
    }
    fn survive(&mut self, row: RC, column: RC) {
        self.board[[row,column]] += 1;
    }
    fn isAlive(&self, row: RC, column: RC) -> bool {
        if self.board[[row,column]] > 0 {
            true
        }
        else {
            false
        }
    }
    fn neighbors(&self, row: RC, column: RC) -> RC {
        let mut total: RC = 0;
        for lr in row.wrapping_sub(1)..row.wrapping_add(1) {
            let mut r = lr;
            if r == MAX {
                if self.wraparound {
                    r = self.rows-1;
                }
                else {
                    continue;
                }
            }
            if r > self.rows-1 {
                if self.wraparound {
                    r = 0;
                }
                else {
                    continue;
                }
            }
            for lc in column.wrapping_sub(1)..column.wrapping_add(1) {
                let mut c = lc;
                if r == row && c == column {
                    continue;
                }
                if c == MAX {
                    if self.wraparound {
                        c = self.cols-1;
                    }
                    else {
                        continue;
                    }
                }
                if self.isAlive(r, c) {
                    total += 1;
                }
            }
        }
    total
    }
}
