use std::io::{self, Write};
mod conways;

fn main() {
    println!("Begin");
    let mut con = conways::Life::new(10, 10);
    con.populate();
    loop {
        for r in 0..con.rows {
            for c in 0..con.cols {
                if con.is_alive(r, c) {
                    print!("+");
                }
                else {
                    print!("-");
                }
            }
            println!("");
            io::stdout().flush().unwrap();
        }
        con.next();

    }
}
