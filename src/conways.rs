extern crate multiarray;

use multiarray::Array2D;
use std::usize::MAX;

type Board = Array2D<u8>;
type RC = usize;

pub struct Life {
    rows: RC,
    cols: RC,
    board: Board,
    wraparound: bool,
}

impl Life {
    pub fn new(rows: RC, cols: RC) -> Self {
        Self {
            rows: rows,
            cols: cols,
            board: Array2D::new([rows, cols], 0),
            wraparound: true,
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
                if self.board[[r,c]] > 0 {
                    total += 1;
                }
            }
        }
    total
    }
}
