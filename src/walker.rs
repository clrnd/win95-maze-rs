use rand;
use cgmath::{vec3, Point3, Vector3};

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

    pub fn to_vec(&self) -> Vector3<f32> {
        match *self {
            Direction::North => vec3(0.0, 0.0, -1.0),
            Direction::East => vec3(1.0, 0.0, 0.0),
            Direction::South => vec3(0.0, 0.0, 1.0),
            Direction::West => vec3(-1.0, 0.0, 0.0)
        }
    }
}

impl<'a> Walker<'a> {
    pub fn new(maze: &Maze, i: usize, j: usize) -> Walker {
        let direction = if maze.south(i, j) {
            Direction::East
        } else {
            Direction::South
        };

        Walker {
            maze: maze,
            direction: direction,
            i: i,
            j: j
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

    pub fn pos(&self) -> (usize, usize) {
        (self.i, self.j)
    }

    pub fn to_point(&self) -> Point3<f32> {
        Point3::new(self.j as f32 + 0.5, 0.0, self.i as f32 + 0.5)
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
