use bevy::prelude::*;

#[derive(Debug, Copy, Clone, Component)]
pub struct OnGameScreen;

#[derive(Debug, Copy, Clone, Component)]
pub enum Rotation {
    // 0 - spawned state;
    R0,
    // R - 90 degrees clockwise from spawned;
    RR,
    // 2- 180 degrees any direction;
    R2,
    // L - 90 degrees counterclockwise from spawned;
    RL,
}

#[derive(Debug, Clone, Copy, Component)]
pub enum FigureType {
    O,
    I,
    S,
    Z,
    L,
    J,
    T,
}

impl rand::distributions::Distribution<FigureType> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> FigureType {
        match rng.gen_range(0..=6) {
            0 => FigureType::O,
            1 => FigureType::I,
            2 => FigureType::S,
            3 => FigureType::Z,
            4 => FigureType::L,
            5 => FigureType::J,
            6 => FigureType::T,
            _ => unreachable!(),
        }
    }
}

impl FigureType {
    /// Get all possible figure types.
    /// Used to implement random generation of
    /// the next figure.
    ///
    /// Checkout TetroBag.
    pub const fn all() -> &'static [FigureType] {
        &[
            FigureType::O,
            FigureType::I,
            FigureType::S,
            FigureType::Z,
            FigureType::L,
            FigureType::J,
            FigureType::T,
        ]
    }
    /// This function returns the dots that represent the figure in its spawned state.
    /// This function should be used only when spawning figures.
    ///
    /// Every point is shown as (x, y);
    pub const fn to_dots(&self) -> &'static [(i32, i32)] {
        match self {
            FigureType::O => &[(0, 0), (1, 0), (0, 1), (1, 1)],
            FigureType::I => &[(0, 0), (1, 0), (2, 0), (3, 0)],
            FigureType::S => &[(0, 0), (1, 0), (1, 1), (2, 1)],
            FigureType::Z => &[(1, 0), (2, 0), (0, 1), (1, 1)],
            FigureType::L => &[(2, 1), (0, 0), (1, 0), (2, 0)],
            FigureType::J => &[(0, 1), (0, 0), (1, 0), (2, 0)],
            FigureType::T => &[(0, 0), (1, 0), (2, 0), (1, 1)],
        }
    }
    // Finds the center point of the figure in its current rotation.
    // This is used to calculate the offset of the figure when rotating it.
    // Every point is shown as (x, y);
    pub const fn center_point(&self, rot: &Rotation) -> (i32, i32) {
        match self {
            FigureType::O => match rot {
                Rotation::R0 => (0, 0),
                Rotation::RR => (0, 1),
                Rotation::R2 => (1, 1),
                Rotation::RL => (1, 0),
            },
            FigureType::L | FigureType::J | FigureType::Z | FigureType::S | FigureType::T => {
                match rot {
                    Rotation::R0 => (1, 0),
                    Rotation::RR => (0, 1),
                    Rotation::R2 => (1, 1),
                    Rotation::RL => (1, 1),
                }
            }
            FigureType::I => match rot {
                Rotation::R0 => (1, 0),
                Rotation::RR => (0, 2),
                Rotation::R2 => (2, 0),
                Rotation::RL => (0, 1),
            },
        }
    }

    pub const fn tests_for_single_rot(&self, rot: &Rotation) -> &'static [(i32, i32)] {
        match self {
            FigureType::L | FigureType::J | FigureType::Z | FigureType::S | FigureType::T => {
                match rot {
                    Rotation::R0 => &[(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                    Rotation::RR => &[(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                    Rotation::R2 => &[(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                    Rotation::RL => &[(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
                }
            }
            FigureType::O => match rot {
                Rotation::R0 => &[(0, 0)],
                Rotation::RR => &[(0, -1)],
                Rotation::R2 => &[(-1, -1)],
                Rotation::RL => &[(-1, 0)],
            },
            FigureType::I => match rot {
                Rotation::R0 => &[(0, 0), (-1, 0), (2, 0), (-1, 0), (2, 0)],
                Rotation::RR => &[(-1, 0), (0, 0), (0, 0), (0, 1), (0, -2)],
                Rotation::R2 => &[(-1, 1), (1, 1), (-2, 1), (1, 0), (-2, 0)],
                Rotation::RL => &[(0, 1), (0, 1), (0, 1), (0, -1), (0, 2)],
            },
        }
    }

    /// This function calculates test offsets,
    /// based on the current and next rotations.
    /// This is generally made to perform wall-kicks and fit tetronomio
    /// in places where they would normally not fit.
    pub fn tests_for_rot(&self, prev: &Rotation, next: &Rotation) -> Vec<(i32, i32)> {
        self.tests_for_single_rot(prev)
            .iter()
            .zip(self.tests_for_single_rot(next))
            .map(|(prev, next)| (prev.0 - next.0, prev.1 - next.1))
            .collect()
    }
}

impl Rotation {
    pub fn left(&self) -> Self {
        match self {
            Rotation::R0 => Rotation::RL,
            Rotation::RL => Rotation::R2,
            Rotation::R2 => Rotation::RR,
            Rotation::RR => Rotation::R0,
        }
    }

    pub fn right(&self) -> Self {
        match self {
            Rotation::R0 => Rotation::RR,
            Rotation::RR => Rotation::R2,
            Rotation::R2 => Rotation::RL,
            Rotation::RL => Rotation::R0,
        }
    }
}

#[derive(Component)]
pub struct ScoreLabel;

#[derive(Debug, Clone, Component)]
pub struct GridCell;

#[derive(Debug, Clone, Component, PartialEq, Eq, Hash)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Component)]
pub struct PlayableTile;
