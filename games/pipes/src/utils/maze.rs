use bevy::utils::HashSet;
use rand::{seq::SliceRandom, Rng};

use super::{direction::Direction, wrapper::wrap_val};

#[derive(Clone, Debug, Default)]
pub struct MazeCell {
    pub connections: Vec<Direction>,
}

#[derive(Clone, Debug)]
pub struct Maze {
    pub grid: Vec<Vec<MazeCell>>,
    pub wrap: bool,
}

pub struct MazeBuilder {
    size: usize,
    avoid_straight: u8,
    prim_percent: u8,
    wrap: bool,
}

impl MazeCell {
    pub fn add_connection(&mut self, direction: Direction) {
        self.connections.push(direction);
        self.connections.sort();
    }
}

impl MazeBuilder {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            avoid_straight: 0,
            prim_percent: 0,
            wrap: false,
        }
    }

    pub fn with_prim_prob(mut self, prob: u8) -> Self {
        self.prim_percent = prob;
        self
    }

    pub fn with_avoid_straight(mut self, prob: u8) -> Self {
        self.avoid_straight = prob;
        self
    }

    pub fn with_wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn build(self) -> Maze {
        Maze::new(
            self.size,
            self.avoid_straight as f64 / 100.,
            self.prim_percent as f64 / 100.,
            self.wrap,
        )
    }
}

impl Maze {
    pub fn builder(size: usize) -> MazeBuilder {
        MazeBuilder::new(size)
    }

    fn new(size: usize, avoid_straight_prob: f64, prim_prob: f64, wrap: bool) -> Self {
        let mut grid = Vec::with_capacity(size);
        for _ in 0..size {
            let mut row = Vec::with_capacity(size);
            for _ in 0..size {
                row.push(MazeCell::default());
            }
            grid.push(row);
        }
        let mut maze = Self { grid, wrap };
        let mut random = rand::thread_rng();
        let start = (
            random.gen_range(0..size) as i32,
            random.gen_range(0..size) as i32,
        );
        let mut visited = HashSet::new();
        let mut stack = Vec::<(i32, i32)>::new();
        stack.push(start);
        visited.insert(start);
        // Slightly modified DFS algorithm.
        while !stack.is_empty() {
            // If we should use Prim's algorithm, we choose a random cell
            // from the stack. If not, we choose the last cell as if we
            // were walking.
            let index = if random.gen_bool(prim_prob) {
                random.gen_range(0..stack.len())
            } else {
                stack.len() - 1
            };
            let Some((x, y)) = stack.iter().nth(index) else {
                break;
            };
            let mut directions = Vec::new();
            // Here we iterate of all the directions and check if they are valid
            // to move to. If they are, we add them to the directions vector.
            for direction in Direction::iter() {
                let (dx, dy) = direction.offset();
                let nx = wrap_val(wrap, *x + dx, 0, size as i32 - 1);
                let ny = wrap_val(wrap, *y + dy, 0, size as i32 - 1);
                if nx >= 0
                    && nx < size as i32
                    && ny >= 0
                    && ny < size as i32
                    && !visited.contains(&(nx, ny))
                {
                    directions.push(*direction);
                }
            }
            let cell = maze.mut_get_cell((*x, *y));
            // Here we check if we should avoid creating straight lines.
            // If we should, we remove the opposite direction of the last
            // connection from the directions vector.
            if random.gen_bool(avoid_straight_prob) && cell.connections.len() == 1 {
                let to_del = cell.connections[0].opposite();
                let new_directions = directions
                    .iter()
                    .copied()
                    .filter(|d| *d != to_del)
                    .collect::<Vec<_>>();
                if !new_directions.is_empty() {
                    directions = new_directions;
                }
            }
            // Here we choose a random direction from the directions vector.
            let Some(direction) = directions.choose(&mut random) else {
                // If we got here, means that there are no valid directions
                // to choose from. So we delete this point from our stack
                // and get to the next one.
                stack.remove(index);
                continue;
            };
            let (dx, dy) = direction.offset();
            let nx = wrap_val(wrap, *x as i32 + dx, 0, size as i32 - 1);
            let ny = wrap_val(wrap, *y as i32 + dy, 0, size as i32 - 1);
            cell.add_connection(*direction);
            let n_cell = maze.mut_get_cell((nx, ny));
            n_cell.add_connection(direction.opposite());
            stack.push((nx, ny));
            visited.insert((nx, ny));
        }
        maze
    }

    fn mut_get_cell<'a>(&'a mut self, (x, y): (i32, i32)) -> &'a mut MazeCell {
        &mut self.grid[y as usize][x as usize]
    }
}

impl ToString for MazeCell {
    fn to_string(&self) -> String {
        if self.connections == [Direction::Up] {
            String::from("╵")
        } else if self.connections == [Direction::Down] {
            String::from("╷")
        } else if self.connections == [Direction::Left] {
            String::from("╴")
        } else if self.connections == [Direction::Right] {
            String::from("╶")
        } else if self.connections == [Direction::Up, Direction::Down] {
            String::from("│")
        } else if self.connections == [Direction::Left, Direction::Right] {
            String::from("─")
        } else if self.connections == [Direction::Up, Direction::Right] {
            String::from("└")
        } else if self.connections == [Direction::Up, Direction::Left] {
            String::from("┘")
        } else if self.connections == [Direction::Down, Direction::Right] {
            String::from("┌")
        } else if self.connections == [Direction::Down, Direction::Left] {
            String::from("┐")
        } else if self.connections == [Direction::Up, Direction::Down, Direction::Left] {
            String::from("┤")
        } else if self.connections == [Direction::Up, Direction::Down, Direction::Right] {
            String::from("├")
        } else if self.connections == [Direction::Up, Direction::Left, Direction::Right] {
            String::from("┴")
        } else if self.connections == [Direction::Down, Direction::Left, Direction::Right] {
            String::from("┬")
        } else if self.connections
            == [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ]
        {
            String::from("┼")
        } else {
            String::from("x")
        }
    }
}

impl ToString for Maze {
    fn to_string(&self) -> String {
        let mut out = String::new();

        for (y, row) in self.grid.iter().enumerate() {
            out += format!("{y:-2} ").as_str();
            for cell in row {
                out += &cell.to_string();
            }
            out.push('\n');
        }

        out
    }
}
