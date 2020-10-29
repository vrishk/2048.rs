use std::cmp::{max, min};
use std::fmt;
use std::io;
use std::time::SystemTime;

// SECTION: Constants

const N: usize = 4;
const START_NUM: i32 = 1;

// SECTION: Helper Functions

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let fmod = |n: f32, d: i32| {
        let r = (n / d as f32) as i32;
        n - (r * d) as f32
    };

    let f = |n: f32| -> u8 {
        let k = fmod(n + h / 60.0, 6);
        let mut sc = if k > 4.0 - k { 4.0 - k } else { k };
        if sc > 1.0 {
            sc = 1.0
        };
        if sc < 0.0 {
            sc = 0.0
        };

        ((v - v * s * sc) * 256.0) as u8
    };

    (f(5.0), f(3.0), f(1.0))
}

// Hue Angle defined by value
fn get_color(n: u32) -> String {
    if n > 0 {
        let col = hsv_to_rgb((n as f32).ln() * 180.0 / 10.0, 0.9, 0.6);
        format!("\u{001b}[48;2;{};{};{}m\u{001b}[37m", col.0, col.1, col.2)
    } else {
        "\u{001b}[48;2;26;77;151m".to_string()
    }
}

// XORShift RNG
fn random() -> usize {
    let mut r = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_micros() as u32;
    r ^= r << 13;
    r ^= r >> 17;
    r ^= r << 5;
    r as usize
}

// SECTION: Main Struct

#[derive(Debug)]
struct Grid {
    dim: usize,
    grid: Vec<Vec<u32>>,
}

impl Grid {
    fn new(dim: usize) -> Self {
        let mut grid = Grid {
            dim: dim,
            grid: vec![vec![0; dim]; dim],
        };

        for _ in 0..START_NUM {
            grid.add();
        }

        grid
    }

    fn add(&mut self) {
        loop {
            let x = random() % self.dim;
            let y = random() % self.dim;
            if self.grid[x][y] == 0 {
                self.grid[x][y] = 2;
                break;
            }
        }
    }

    fn rotate(&mut self) {
        for i in 0..self.dim / 2 {
            for j in i..(self.dim - i - 1) {
                let temp = self.grid[i][j];
                self.grid[i][j] = self.grid[j][self.dim - i - 1];
                self.grid[j][self.dim - i - 1] = self.grid[self.dim - i - 1][self.dim - j - 1];
                self.grid[self.dim - i - 1][self.dim - j - 1] = self.grid[self.dim - j - 1][i];
                self.grid[self.dim - j - 1][i] = temp;
            }
        }
    }

    fn slide(&mut self) {
        for i in 0..self.dim {
            let mut j: usize = 0;
            while j < self.grid[i].len() {
                if self.grid[i][j] == 0 {
                    self.grid[i].remove(j);
                } else {
                    j += 1;
                }
            }

            let mut j: usize = 0;

            while j + 1 < self.grid[i].len() {
                if self.grid[i][j] == self.grid[i][j + 1] {
                    self.grid[i][j] *= 2;
                    self.grid[i].remove(j + 1);
                } else {
                    j += 1;
                }
            }
            for _ in 0..(self.dim - self.grid[i].len()) {
                self.grid[i].push(0);
            }
        }
    }

    fn up(&mut self) {
        self.rotate();
        self.slide();
        self.rotate();
        self.rotate();
        self.rotate();
    }

    fn down(&mut self) {
        self.rotate();
        self.rotate();
        self.rotate();
        self.slide();
        self.rotate();
    }

    fn right(&mut self) {
        self.rotate();
        self.rotate();
        self.slide();
        self.rotate();
        self.rotate();
    }

    fn left(&mut self) {
        self.slide();
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res: String = "\u{001b}[2J\u{001b}[1;1H".to_string();
        // let mut res: String = "".to_string();
        for j in 0..self.grid.len() {
            for i in 0..self.grid[j].len() {
                res = [
                    res,
                    format!("{}{}\u{001b}[0m", get_color(self.grid[j][i]), " ".repeat(7)),
                ]
                .join("");
            }
            res += "\n";
            for i in 0..self.grid[j].len() {
                let val = if self.grid[j][i] > 0 {
                    self.grid[j][i].to_string()
                } else {
                    ".".to_string()
                };
                res = [
                    res,
                    format!(
                        "{}{}{}{}\u{001b}[0m",
                        get_color(self.grid[j][i]),
                        " ".repeat(4 - (val.len() + 1) / 2),
                        val,
                        " ".repeat(3 - val.len() + (val.len() + 1) / 2)
                    ),
                ]
                .join("");
            }
            res += "\n";
            for i in 0..self.grid[j].len() {
                res = [
                    res,
                    format!("{}{}\u{001b}[0m", get_color(self.grid[j][i]), " ".repeat(7)),
                ]
                .join("");
            }
            res += "\n";
        }
        write!(f, "{}", res)
    }
}

// SECTION: Main Process

fn main() {
    let mut grid = Grid::new(N);

    loop {
        grid.add();
        println!("{}", grid);
        let mut inp = String::new();
        io::stdin()
            .read_line(&mut inp)
            .expect("ERROR: Improper input.");
        match inp.as_str() {
            "w\n" => grid.up(),
            "s\n" => grid.down(),
            "a\n" => grid.left(),
            "d\n" => grid.right(),
            "q\n" => break,
            _ => print!("Invalid Input"),
        };
    }
}
