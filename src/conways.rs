extern crate multiarray;
extern crate rand;

use multiarray::Array2D;
use std::usize::MAX;
use std::vec::Vec;
use rand::{thread_rng, Rng};

type Cell = u8;
type Board = Array2D<Cell>;
type RC = usize;
type Rule = Vec<RC>;

pub struct Life {
    pub rows: RC,
    pub cols: RC,
    pub wraparound: bool,
    board: Board,
    birth: Rule,
    survive: Rule,
}

struct Change {
    row: RC,
    column: RC,
    state: Cell,
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
    fn kill(&self, change: &mut Vec<Change>, row: RC, column: RC) {
        //self.board[[row,column]] = 0;
        change.push(Change { row: row, column: column, state: 0, });
    }
    fn survive(&self, change: &mut Vec<Change>, row: RC, column: RC) {
        //self.board[[row,column]] += 1;
        let newstate = self.board[[row,column]] + 1;
        change.push(Change { row: row, column: column, state: newstate, });
    }
    pub fn is_alive(&self, row: RC, column: RC) -> bool {
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
                if self.is_alive(r, c) {
                    total += 1;
                }
            }
        }
    total
    }
    pub fn next(&mut self) {
        let mut change: Vec<Change> = Vec::new();
        for r in 0..self.rows {
            for c in 0..self.cols {
                let n = self.neighbors(r, c);
                if self.is_alive(r, c) {
                    let mut survive = false;
                    for s in &self.survive {
                        if n == *s {
                            survive = true;
                            continue;
                        }
                    }
                    if survive {
                        self.survive(&mut change, r, c);
                    }
                    else {
                        self.kill(&mut change, r, c);
                    }
                }
                else {
                    let mut live = false;
                    for b in &self.birth {
                        if n == *b {
                            live = true;
                            continue;
                        }

                    }
                    if live {
                        self.survive(&mut change, r, c);
                    }
                }
            }
        }
        // Apply changes
        for c in change {
            self.board[[c.row, c.column]] = c.state;
        }
    }
    pub fn populate(&mut self) {
        let mut rng = thread_rng();
        for r in 0..self.rows {
            for c in 0..self.cols {
                self.board[[r, c]] = rng.gen_range(0, 1);
            }
        }
    }
}
