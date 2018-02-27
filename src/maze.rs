use rand;

const N: u8 = 0b0001;
const E: u8 = 0b0010;
const S: u8 = 0b0100;
const W: u8 = 0b1000;

#[derive(Debug)]
pub struct Maze {
    pub grid: Vec<Vec<u8>>,
    pub width: usize,
    pub height: usize
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {

        let mut grid = Vec::with_capacity(height);

        for _ in 0..height {
            let mut coll = Vec::with_capacity(width);
            for _ in 0..width {
                coll.push(0);
            }
            grid.push(coll);
        }

        let mut maze = Maze { grid: grid,
                              width: width,
                              height: height };

        carve_from(0, 0, &mut maze);

        maze
    }

    pub fn print(&self) {
        print!(" ");
        for _ in 0..self.width*2 - 3 { print!("_") }
        println!();
        for i in 0..self.width - 1 {
            print!("|");
            for j in 0..self.height-1 {
                if self.grid[i][j] & S != 0 {
                    print!(" ")
                } else {
                    print!("_")
                };
                if self.grid[i][j] & E != 0 {
                    if (self.grid[i][j] | self.grid[i][j+1]) & S != 0 {
                        print!(" ")
                    } else {
                        print!("_")
                    }
                } else {
                    print!("|")
                };
            }
            println!("");
        }
    }

    pub fn north(&self, i: usize, j: usize) -> bool {
        self.grid[i][j] & N == 0
    }

    pub fn east(&self, i: usize, j: usize) -> bool {
        self.grid[i][j] & E == 0
    }

    pub fn south(&self, i: usize, j: usize) -> bool {
        self.grid[i][j] & S == 0
    }

    pub fn west(&self, i: usize, j: usize) -> bool {
        self.grid[i][j] & W == 0
    }
}
fn carve_from(cx: usize, cy: usize, maze: &mut Maze) {
    let mut directions: [u8; 4] = [N, E, S, W];
    directions.sort_unstable_by_key(|_| rand::random::<u8>());

    for d in &directions {
        let nx = match *d {
            N => cx,
            E => cx + 1,
            S => cx,
            W => if 0 < cx { cx - 1 } else { continue },
            _ => panic!()
        };
        let ny = match *d {
            N => if 0 < cy { cy - 1 } else { continue },
            E => cy,
            S => cy + 1,
            W => cy,
            _ => panic!()
        };

        let valid_x = nx < (maze.width - 1);
        let valid_y = ny < (maze.height - 1);

        if valid_x && valid_y && maze.grid[ny][nx] == 0 {
            maze.grid[cy][cx] |= d;
            maze.grid[ny][nx] |= match *d {
                N => S,
                E => W,
                S => N,
                W => E,
                _ => panic!()
            };
            carve_from(nx, ny, maze);
        }
    }
}
