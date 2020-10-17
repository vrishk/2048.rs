use std::fmt;
use std::io;
use std::time::SystemTime;

// SECTION: Constants

const N: usize = 4;
const START_NUM: i32 = 3;

// SECTION: Helper Functions

fn get_color(n: u32) -> String {

    "\u{001b}[48;2;0;0;125m".to_string()
}

fn random() -> usize {
    SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .unwrap()
    .as_micros() as usize
}

// SECTION: Main Struct

#[derive(Debug)]
struct Grid {
    dim: usize,
    grid: Vec<Vec<u32>>,
}

impl Grid {
    fn new(dim: usize) -> Self {
        let mut grid = vec![vec![0; dim]; dim];
        
        for _ in 0..START_NUM {
            let x = random() % 4;
            let y = random() % 4;
            grid[x][y] = 2;
        }

        Grid {dim: dim, grid: grid}
    }

    fn rotate(&mut self) {
        for i in 0..self.dim/2 {
            for j in i..(self.dim-i-1) {
                let temp = self.grid[i][j];
                self.grid[i][j] = self.grid[j][self.dim-i-1];
                self.grid[j][self.dim-i-1] = self.grid[self.dim-i-1][self.dim-j-1];
                self.grid[self.dim-i-1][self.dim-j-1] = self.grid[self.dim-j-1][i];
                self.grid[self.dim-j-1][i] = temp;
            }
        }
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
    println!("{}", grid);

    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("ERROR: Improper input.");
    
    grid.rotate();
    println!("{}", grid);
}
