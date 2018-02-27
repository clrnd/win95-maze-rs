use rand;

use maze::Maze;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West
}

pub struct Walker<'a> {
    maze: &'a Maze,
    pub direction: Direction,
    pub i: usize,
    pub j: usize
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East
        }
    }
}

impl<'a> Walker<'a> {
    pub fn new(maze: &Maze) -> Walker {
        let direction = if maze.south(0, 0) {
            Direction::East
        } else {
            Direction::South
        };

        Walker {
            maze: maze,
            direction: direction,
            i: 0,
            j: 0
        }
    }

    fn open(&self, direction: &Direction) -> bool {
        !match *direction {
            Direction::North => self.maze.north(self.i, self.j),
            Direction::East => self.maze.east(self.i, self.j),
            Direction::South => self.maze.south(self.i, self.j),
            Direction::West => self.maze.west(self.i, self.j)
        }

    }

    pub fn next(&mut self) {
        let mut directions: Vec<Direction> =
            vec![Direction::North,
                 Direction::East,
                 Direction::South,
                 Direction::West]
            .into_iter()
            .filter(|d| self.open(d))
            .collect();

        directions.sort_unstable_by_key(|_| rand::random::<u8>());

        for d in &directions {
            if self.open(d) && (self.direction != d.opposite() ||
                                directions.len() == 1) {
                match *d {
                    Direction::North => self.i -= 1,
                    Direction::East => self.j += 1,
                    Direction::South => self.i += 1,
                    Direction::West => self.j -= 1
                }
                self.direction = *d;
                return;
            }
        }
    }
}
