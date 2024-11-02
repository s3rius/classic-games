use bevy::prelude::*;

use super::{
    components::{FigureType, GridCell, GridPosition, PlayableTile, Rotation},
    timers::GravityTimer,
};

pub fn stop_gravity(
    is_debug: Res<crate::state::Debug>,
    mut grav_timer: ResMut<GravityTimer>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if !is_debug.enabled {
        return;
    }
    if input.just_pressed(KeyCode::KeyG) {
        if grav_timer.paused() {
            grav_timer.unpause();
        } else {
            grav_timer.pause();
        }
    }
}

pub fn center_point(
    is_debug: Res<crate::state::Debug>,
    mut grid_query: Query<(&GridPosition, &mut BackgroundColor), With<GridCell>>,
    play_tiles_query: Query<(&GridPosition, &FigureType, &Rotation), With<PlayableTile>>,
) {
    if !is_debug.enabled {
        return;
    }
    let x_offset = play_tiles_query
        .iter()
        .map(|(pos, _, _)| pos.x as i32)
        .min()
        .unwrap_or(0);
    let y_offset = play_tiles_query
        .iter()
        .map(|(pos, _, _)| pos.y as i32)
        .min()
        .unwrap_or(0);
    let mut center_gp = GridPosition { x: 0, y: 0 };
    for (gp, ft, rot) in play_tiles_query.iter() {
        let (center_x, center_y) = ft.center_point(rot);
        if gp.x as i32 - x_offset == center_x && gp.y as i32 - y_offset == center_y {
            center_gp = gp.clone();
        }
    }

    for (gp, mut bg_col) in grid_query.iter_mut() {
        if *gp == center_gp {
            *bg_col = BackgroundColor(Color::WHITE);
        }
    }
}
