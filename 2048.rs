use std::fmt;

// SECTION: Constants

const N: usize = 4;

// SECTION: Helper Functions

fn get_color(n: u32) -> String {
    "\u{001b}[48;2;0;0;125m".to_string()
}

// SECTION: Main Struct

#[derive(Debug)]
struct Grid {
    dim: usize,
    grid: Vec<Vec<u32>>,
}

impl Grid {
    fn new(dim: usize) -> Self {
        let grid = vec![vec![0; dim]; dim];
        Grid {dim: dim, grid: grid}
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res: String = String::new();
        for j in 0..self.grid.len() {
            for i in 0..self.grid[j].len() {
                res = [
                    res, 
                    format!(
                        "{}{}\u{001b}[0m", 
                        get_color(self.grid[j][i]), 
                        " ".repeat(7)
                    )
                ].join("");
            }
            res += "\n";
            for i in 0..self.grid[j].len() {
                res = [
                    res,
                    format!(
                        "{}{}{}{}\u{001b}[0m", 
                        get_color(self.grid[j][i]),
                        " ".repeat(3),
                        self.grid[j][i],
                        " ".repeat(3)
                    )
                ].join("");
            }
            res += "\n";
            for i in 0..self.grid[j].len() {
                res = [
                    res, 
                    format!(
                        "{}{}\u{001b}[0m", 
                        get_color(self.grid[j][i]), 
                        " ".repeat(7)
                    )
                ].join("");
            }
            res += "\n";
        }
        writeln!(f, "{}", res)
    }
}

// SECTION: Main Process

fn main() {
    let mut grid = Grid::new(N);
    println!("{}", grid)
}
