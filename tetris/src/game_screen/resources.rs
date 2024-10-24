use std::collections::VecDeque;

use bevy::prelude::*;
use rand::seq::SliceRandom;

use super::components::FigureType;

// Indicates wether soft drop is active or not.
#[derive(Debug, Clone, Default, Resource)]
pub struct SoftDrop {
    pub active: bool,
}

#[derive(Debug, Clone, Resource, Deref, DerefMut)]
pub struct GameBoard(pub Vec<Vec<bool>>);

#[derive(Debug, Default, Clone, Resource)]
pub struct TetroBag {
    bag: VecDeque<FigureType>,
}

impl GameBoard {
    pub fn new() -> Self {
        Self(vec![vec![false; 10]; 20])
    }

    pub fn reset(&mut self) {
        for row in self.iter_mut() {
            for cell in row.iter_mut() {
                *cell = false;
            }
        }
    }

    /// Check if the selected position is available.
    /// Returns true in case if it is.
    pub fn check(&self, pos: (i32, i32)) -> bool {
        if pos.1 < 0 || pos.1 >= self.len() as i32 {
            return false;
        }
        if pos.0 < 0 || pos.0 >= self[0].len() as i32 {
            return false;
        }
        !self[pos.1 as usize][pos.0 as usize]
    }

    pub fn occupy(&mut self, pos: (i32, i32)) {
        if pos.1 < 0 || pos.1 >= self.len() as i32 {
            return;
        }
        if pos.0 < 0 || pos.0 >= self[0].len() as i32 {
            return;
        }
        self[pos.1 as usize][pos.0 as usize] = true;
    }

    pub fn get_max_y(&self, x: i32) -> usize {
        if x < 0 || x >= self[0].len() as i32 {
            return 0;
        }
        self.iter()
            .enumerate()
            .map(|(y, xs)| if xs[x as usize] { y } else { 0 })
            .max()
            .unwrap_or(0)
    }

    /// Get the maximum y value for a list of x values.
    /// Used to perform HardDrop of a figure.
    pub fn get_max_y_many(&self, xs: impl Iterator<Item = i32>) -> usize {
        xs.map(|x| self.get_max_y(x)).max().unwrap_or(0)
    }

    /// This function is used to clear filled
    /// lines of a game board. Plus it shifts
    /// the lines above the cleared lines down.
    /// It returns the number of lines cleared.
    pub fn clear_lines(&mut self) -> usize {
        let mut cleared = 0;
        let mut y = self.len() as i32 - 1;
        while y >= 0 {
            if self[y as usize].iter().all(|x| *x) {
                self.remove(y as usize);
                self.push(vec![false; 10]);
                cleared += 1;
            } else {
                y -= 1;
            }
        }
        cleared
    }
}

impl TetroBag {
    /// Get next figure from the bag.
    pub fn draw_next(&mut self) -> FigureType {
        if self.bag.len() <= 3 {
            let mut new_figs = FigureType::all().to_vec();
            new_figs.shuffle(&mut rand::thread_rng());
            for fig in new_figs {
                self.bag.push_back(fig);
            }
        }
        self.bag.pop_front().expect("This should not have happened")
    }

    pub fn reset(&mut self) {
        self.bag.clear();
    }
}
