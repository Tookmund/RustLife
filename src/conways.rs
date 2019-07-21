extern crate multiarray;

use multiarray::Array2D;

type Board = Array2D<u8>;

pub struct Life {
    rows: usize,
    cols: usize,
    board: Board,
    wraparound: bool,
}

impl Life {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows: rows,
            cols: cols,
            board: Array2D::new([rows, cols], 0),
            wraparound: true,
        }
    }

}
