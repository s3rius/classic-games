use bevy::math::Quat;
use bevy::prelude::Resource;
use bevy::utils::HashSet;
use rand::Rng;

use crate::utils::direction::Direction;
use crate::utils::maze::{Maze, MazeCell};
use crate::utils::wrapper::wrap_val;

#[derive(Clone, Copy, Debug)]
pub enum PipeType {
    DeadEnd,
    Straight,
    Elbow,
    Tee,
    Cross,
}

#[derive(Clone, Copy, Debug)]
pub enum Rotation {
    /// 0 degrees
    R0,
    /// 90 degrees (clockwise)
    RR,
    /// 180 degrees (180 degrees any direction)
    R2,
    /// 270 degrees (anticlockwise)
    RL,
}

#[derive(Clone, Debug)]
pub struct PipePart {
    pub pipe_type: PipeType,
    pub rotation: Rotation,
}

#[derive(Debug, Default, Resource)]
pub struct GameBoard {
    pub grid: Vec<Vec<PipePart>>,
    pub wrap: bool,
    pub connected: HashSet<(usize, usize)>,
}

impl GameBoard {
    pub fn new() -> Self {
        Self {
            grid: vec![],
            wrap: false,
            connected: HashSet::new(),
        }
    }

    pub fn generate(&mut self, maze: &Maze) {
        self.wrap = maze.wrap;
        self.grid.clear();
        let mut random = rand::thread_rng();
        for maze_row in maze.grid.iter() {
            let mut pipe_row = Vec::with_capacity(maze_row.len());
            for cell in maze_row {
                let mut pipe_part = PipePart::from(cell);
                pipe_part.rotation = random.gen::<Rotation>();
                pipe_row.push(pipe_part);
            }
            self.grid.push(pipe_row);
        }
        self.recalculate_connected();
    }

    pub fn is_solved(&self) -> bool {
        self.grid.len().pow(2) == self.connected.len()
    }

    pub fn recalculate_connected(&mut self) {
        self.connected = HashSet::new();
        let center = self.grid.len() / 2;
        let current = (center, center);
        let mut stack = vec![current];
        let max_val = self.grid.len() as i32 - 1;
        loop {
            let Some((x, y)) = stack.pop() else {
                break;
            };
            self.connected.insert((x, y));
            let directions = self.grid[y][x].get_directions();
            for direction in directions {
                let (dx, dy) = direction.offset();
                let nx = wrap_val(self.wrap, x as i32 + dx, 0, max_val);
                let ny = wrap_val(self.wrap, y as i32 + dy, 0, max_val);
                if nx < 0 || ny < 0 || nx > max_val || ny > max_val {
                    continue;
                }
                if self.connected.contains(&(nx as usize, ny as usize)) {
                    continue;
                }
                if self.grid[ny as usize][nx as usize]
                    .get_directions()
                    .contains(&direction.opposite())
                {
                    stack.push((nx as usize, ny as usize));
                }
            }
        }
    }

    pub fn rotate(&mut self, x: usize, y: usize) {
        self.grid[y][x].rotation = self.grid[y][x].rotation.next();
        self.recalculate_connected();
    }
}

impl PipePart {
    pub const fn get_directions(&self) -> &'static [Direction] {
        self.pipe_type.get_directions(self.rotation)
    }
}

impl Rotation {
    pub fn to_radians(&self) -> f32 {
        match self {
            // This is anti-clockwise rotation in radians.
            Self::R0 => (360f32 - 0f32).to_radians(),
            Self::RR => (360f32 - 90f32).to_radians(),
            Self::R2 => (360f32 - 180f32).to_radians(),
            Self::RL => (360f32 - 270f32).to_radians(),
        }
    }

    pub fn to_quat(&self) -> Quat {
        Quat::from_rotation_z(self.to_radians())
    }

    pub fn next(&self) -> Self {
        match self {
            Self::R0 => Self::RR,
            Self::RR => Self::R2,
            Self::R2 => Self::RL,
            Self::RL => Self::R0,
        }
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::R0, Self::RR, Self::R2, Self::RL].iter().copied()
    }
}

impl PipeType {
    pub const fn get_directions(&self, rot: Rotation) -> &'static [Direction] {
        match self {
            Self::DeadEnd => match rot {
                Rotation::R0 => &[Direction::Up],
                Rotation::RR => &[Direction::Right],
                Rotation::R2 => &[Direction::Down],
                Rotation::RL => &[Direction::Left],
            },
            Self::Straight => match rot {
                Rotation::R0 => &[Direction::Up, Direction::Down],
                Rotation::RR => &[Direction::Left, Direction::Right],
                Rotation::R2 => &[Direction::Up, Direction::Down],
                Rotation::RL => &[Direction::Left, Direction::Right],
            },
            Self::Elbow => match rot {
                Rotation::R0 => &[Direction::Up, Direction::Right],
                Rotation::RR => &[Direction::Down, Direction::Right],
                Rotation::R2 => &[Direction::Down, Direction::Left],
                Rotation::RL => &[Direction::Up, Direction::Left],
            },
            Self::Tee => match rot {
                Rotation::R0 => &[Direction::Up, Direction::Left, Direction::Right],
                Rotation::RR => &[Direction::Up, Direction::Down, Direction::Right],
                Rotation::R2 => &[Direction::Down, Direction::Left, Direction::Right],
                Rotation::RL => &[Direction::Up, Direction::Down, Direction::Left],
            },
            Self::Cross => &[
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ],
        }
    }
}

impl From<&MazeCell> for PipePart {
    fn from(value: &MazeCell) -> Self {
        for pipe_type in [
            PipeType::DeadEnd,
            PipeType::Straight,
            PipeType::Elbow,
            PipeType::Tee,
            PipeType::Cross,
        ] {
            for rotation in Rotation::iter() {
                if pipe_type.get_directions(rotation) == value.connections {
                    return Self {
                        pipe_type,
                        rotation,
                    };
                }
            }
        }
        unreachable!("Unknown maze-cell {:?}", value);
    }
}

impl rand::distributions::Distribution<Rotation> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Rotation {
        let range = rng.gen_range(0..=3);
        match range {
            0 => Rotation::R0,
            1 => Rotation::RR,
            2 => Rotation::R2,
            3 => Rotation::RL,
            _ => unreachable!(),
        }
    }
}
