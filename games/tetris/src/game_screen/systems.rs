use std::time::Duration;

use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

use crate::{
    consts,
    state::{GameState, Score},
    utils::rotations::{rotate_left, rotate_right},
};

use super::{
    components::{
        FigureType, GridCell, GridPosition, OnGameScreen, PlayableTile, Rotation, ScoreLabel,
    },
    events,
    resources::{GameBoard, SoftDrop, TetroBag},
    timers::{self, LockdownTimer},
};

pub fn reset_game(
    mut score: ResMut<Score>,
    mut board: ResMut<GameBoard>,
    mut bag: ResMut<TetroBag>,
) {
    score.score = 0;
    score.lines_cleared = 0;
    board.reset();
    bag.reset();
}

pub fn spawn_figure(mut commands: Commands, mut bag: ResMut<TetroBag>) {
    let fig = bag.draw_next();
    let fig_dots = fig.to_dots();
    for (x, y) in fig_dots {
        commands.spawn((
            fig,
            PlayableTile,
            Rotation::R0,
            GridPosition {
                x: x + consts::START_X_POSITION,
                y: y + consts::START_Y_POSITION,
            },
        ));
    }
}

pub fn setup_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    grid_template_columns: vec![GridTrack::percent(60.), GridTrack::percent(40.)],
                    ..default()
                },
                ..default()
            },
            OnGameScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        justify_self: JustifySelf::Center,
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        max_width: Val::Px(600.),
                        aspect_ratio: Some(1.0),
                        display: Display::Grid,
                        align_items: AlignItems::Center,
                        grid_template_columns: RepeatedGridTrack::percent(10, 10.),
                        grid_template_rows: RepeatedGridTrack::percent(20, 5.),
                        grid_auto_flow: GridAutoFlow::Column,
                        ..default()
                    },
                    ..default()
                })
                .with_children(setup_main_grid_cells);
            // Node for score and upcoming figures
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        padding: UiRect {
                            top: Val::Percent(10.),
                            left: Val::Percent(4.),
                            ..default()
                        },
                        flex_wrap: FlexWrap::Wrap,
                        row_gap: Val::Percent(4.),
                        align_content: AlignContent::Start,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        ScoreLabel,
                        TextBundle {
                            text: Text::from_section(
                                "",
                                TextStyle {
                                    font: asset_server.load(cgc_lib::COMMON_FONT_NAME),
                                    color: consts::FOREGROUND_COLOR,
                                    ..default()
                                },
                            ),
                            ..default()
                        },
                    ));
                });
        });
}

fn setup_main_grid_cells(spawner: &mut ChildBuilder) {
    for x in 0..10 {
        for y in 0..20 {
            spawner.spawn((
                GridCell,
                GridPosition { x, y: 19 - y },
                NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        width: Val::Percent(95.),
                        height: Val::Percent(95.),
                        border: UiRect::all(Val::Px(1.)),
                        padding: UiRect::all(Val::Percent(10.)),
                        ..default()
                    },
                    border_radius: BorderRadius::all(Val::Percent(10.)),
                    border_color: BorderColor(consts::DIMMED_FOREGROUND_COLOR),
                    ..default()
                },
            ));
        }
    }
}

pub fn update_score_label(
    mut label_query: Query<&mut Text, With<ScoreLabel>>,
    score: Res<Score>,
    asset_server: Res<AssetServer>,
) {
    if !score.is_changed() {
        return;
    }
    let mut label = label_query.single_mut();
    *label = Text::from_section(
        format!(
            "Score:\n{:0>12}\nLines: {:0>5}",
            score.score, score.lines_cleared
        ),
        TextStyle {
            font_size: 16.,
            font: asset_server.load(cgc_lib::COMMON_FONT_NAME),
            color: consts::FOREGROUND_COLOR,
            ..default()
        },
    );
}

pub fn color_grid(
    mut grid_query: Query<(&mut BackgroundColor, &GridPosition), With<GridCell>>,
    play_tiles_query: Query<&GridPosition, With<PlayableTile>>,
    board: Res<GameBoard>,
) {
    let play_tiles = play_tiles_query.iter().collect::<HashSet<_>>();
    for (mut color, position) in &mut grid_query {
        if !board.check((position.x, position.y)) || play_tiles.contains(&position) {
            *color = BackgroundColor(consts::LIGHT_FOREGROUND_COLOR);
        } else {
            *color = BackgroundColor(consts::BACKGROUND_COLOR);
        }
    }
}

pub fn handle_inputs(
    key: Res<ButtonInput<KeyCode>>,
    mut soft_drop: ResMut<SoftDrop>,
    mut horizontall_moves: EventWriter<events::MoveTetronomioHorizontally>,
    mut rot_events: EventWriter<events::RotateTetronomio>,
    mut hard_drop_events: EventWriter<events::HardDrop>,
) {
    if key.just_pressed(KeyCode::ArrowLeft) || key.just_pressed(KeyCode::KeyH) {
        horizontall_moves.send(events::MoveTetronomioHorizontally { right: false });
    }
    if key.just_pressed(KeyCode::ArrowRight) || key.just_pressed(KeyCode::KeyL) {
        horizontall_moves.send(events::MoveTetronomioHorizontally { right: true });
    }
    if key.just_pressed(KeyCode::Space) {
        hard_drop_events.send(events::HardDrop);
    }
    if key.just_pressed(KeyCode::ArrowUp) || key.just_pressed(KeyCode::KeyK) {
        let shift_pressed = key.pressed(KeyCode::ShiftLeft) || key.pressed(KeyCode::ShiftRight);
        rot_events.send(events::RotateTetronomio {
            // If shift is pressed, we rotate counter-clockwise.
            clockwise: !shift_pressed,
        });
    }

    if key.pressed(KeyCode::ArrowDown) || key.pressed(KeyCode::KeyJ) {
        if !soft_drop.active {
            soft_drop.active = true;
        }
    } else if soft_drop.active {
        soft_drop.active = false;
    }
}

pub fn lockdown_tiles(
    mut timer: ResMut<LockdownTimer>,
    time: Res<Time>,
    mut board: ResMut<GameBoard>,
    mut tiles: Query<(&mut FigureType, &mut Rotation, &mut GridPosition), With<PlayableTile>>,
    mut state: ResMut<NextState<GameState>>,
    mut bag: ResMut<TetroBag>,
) {
    if timer.tick(time.delta()).just_finished() {
        let can_move_down = tiles
            .iter()
            .all(|(_, _, pos)| board.check((pos.x, pos.y - 1)));
        if can_move_down {
            return;
        }
        // Here we should lock the tiles in place.
        let next_figure = bag.draw_next();
        let new_dots = next_figure.to_dots();
        for (i, (mut fig_type, mut rotation, mut grid_pos)) in tiles.iter_mut().enumerate() {
            board.occupy((grid_pos.x, grid_pos.y));
            *fig_type = next_figure;
            *rotation = Rotation::R0;
            *grid_pos = GridPosition {
                x: new_dots[i].0 + consts::START_X_POSITION,
                y: new_dots[i].1 + consts::START_Y_POSITION,
            };
            if !board.check((grid_pos.x, grid_pos.y)) {
                state.set(GameState::GameOver);
            }
        }
    }
}

pub fn rotate_tetronomio(
    mut event_reader: EventReader<events::RotateTetronomio>,
    mut playable_tiles: Query<(&mut GridPosition, &FigureType, &mut Rotation), With<PlayableTile>>,
    mut lockdown_timer: ResMut<timers::LockdownTimer>,
    board: Res<GameBoard>,
) {
    // We iterate over all rotation events,
    // but actually there should be no more than 1
    // event at a time. Because no other systems except for input are sending this event.
    for event in event_reader.read() {
        // Offsets of the figure bounding box.
        let x_offset = playable_tiles
            .iter()
            .map(|(pos, _, _)| pos.x as i32)
            .min()
            .unwrap_or(0);
        let y_offset = playable_tiles
            .iter()
            .map(|(pos, _, _)| pos.y as i32)
            .min()
            .unwrap_or(0);
        let Some((_, fig, rot)) = playable_tiles.iter().next() else {
            // This statement should never be reached.
            // If it does, it means that there are no playable tiles (which should never be true).
            continue;
        };
        let mut valid_pos = HashMap::<(i32, i32), (i32, i32)>::with_capacity(4);
        let mut can_move = false;
        let next_rot = if event.clockwise {
            rot.right()
        } else {
            rot.left()
        };
        let (center_x, center_y) = fig.center_point(&rot);
        // Here we iterate over rotation tests
        // and try to fit the piece in.
        for (test_x, test_y) in fig.tests_for_rot(&rot, &next_rot) {
            // For each test, we iterate over all tiles controlled by a player.
            // If we find a valid position for all tiles, we can move the figure.
            let mut has_error = false;
            for (tile, _, _) in playable_tiles.iter() {
                // Here we subtract the offset of the bounding box and the center point
                // to get the relative position of the tile to the center point.
                let (new_x, new_y) = if event.clockwise {
                    rotate_right(tile.x - x_offset - center_x, tile.y - y_offset - center_y)
                } else {
                    rotate_left(tile.x - x_offset - center_x, tile.y - y_offset - center_y)
                };
                // Here we add the offset of the bounding box and the center point back
                // to get the absolute position of the tile.
                // And also we add the test offset to the tile position to perform
                // wall-kick if possible.
                let new_mapped_x = new_x + x_offset + center_x + test_x;
                let new_mapped_y = new_y + y_offset + center_y + test_y;
                if board.check((new_mapped_x, new_mapped_y)) {
                    // If this point has passed the test, we add it to our
                    // mapping. Which maps original position to the new position.
                    valid_pos.insert((tile.x, tile.y), (new_mapped_x, new_mapped_y));
                    continue;
                }
                // If you reach to this point, then this test is failed, and we should
                // continue to the next test.
                has_error = true;
            }
            // This means that all points can move to desired location.
            // We move if that's true.
            if !has_error {
                can_move = true;
                break;
            }
        }
        // We cannot perform any rotation, so we do nothing.
        if !can_move {
            continue;
        }
        if !lockdown_timer.finished() {
            lockdown_timer.reset();
        }
        // Here we move the tiles to their new locations, also updating rotation data.
        for (mut tile, _, mut rot) in playable_tiles.iter_mut() {
            *rot = next_rot;
            if let Some((new_x, new_y)) = valid_pos.get(&(tile.x, tile.y)) {
                tile.x = *new_x;
                tile.y = *new_y;
            }
        }
    }
}

pub fn move_tetronomio_horizontally(
    mut event_reader: EventReader<events::MoveTetronomioHorizontally>,
    mut playable_tiles: Query<(&mut GridPosition, &FigureType, &mut Rotation), With<PlayableTile>>,
    board: Res<GameBoard>,
) {
    // We iterate over all rotation events,
    // but actually there should be no more than 1
    // event at a time. Because no other systems except for input are sending this event.
    for event in event_reader.read() {
        let mut delta = -1;
        // If we are moving to the right,
        // we change direction to increasing x.
        if event.right {
            delta = 1;
        }
        let can_move = playable_tiles
            .iter()
            .all(|(pos, _, _)| board.check((pos.x + delta, pos.y)));
        if !can_move {
            continue;
        }
        for (mut tile, _, _) in playable_tiles.iter_mut() {
            tile.x += delta;
        }
    }
}

pub fn hard_drop(
    mut playable_tiles: Query<&mut GridPosition, With<PlayableTile>>,
    board: Res<GameBoard>,
    mut score: ResMut<Score>,
) {
    let all_xs = playable_tiles.iter().map(|pos| pos.x);
    // Here we find maximum y for all x positions,
    // So we know exactly where we should place our tile.
    let max_board_y = board.get_max_y_many(all_xs) as i32;
    let bb_y = playable_tiles.iter().map(|pos| pos.y).min().unwrap_or(0);
    let mut grid_delta = 0;
    for mut pos in playable_tiles.iter_mut() {
        let new_y = max_board_y + 1 + (pos.y - bb_y);
        grid_delta += (new_y - pos.y).abs() as usize;
        pos.y = new_y;
    }
    if grid_delta > 0 {
        score.score += grid_delta * 3;
    }
}

/// This system is responsible for moving the tiles down.
///
/// Basically, when the gravity timer hits, we check if we can
/// move tiles down. If we do, then we reset the lockdown timer so
/// tiles won't be locked in place.
///
/// If we cannot move down any further, we start the lockdown timer.
pub fn gravity(
    mut tiles: Query<&mut GridPosition, With<PlayableTile>>,
    board: Res<GameBoard>,
    time: Res<Time>,
    mut gravity_timer: ResMut<timers::GravityTimer>,
    mut lockdown_timer: ResMut<timers::LockdownTimer>,
    soft_drop: Res<SoftDrop>,
    mut score: ResMut<Score>,
) {
    let mulitplier = if soft_drop.active {
        consts::SOFT_DROP_MULTIPLER
    } else {
        1
    };
    if !gravity_timer.tick(time.delta() * mulitplier).finished() {
        return;
    }
    let can_move_down = tiles.iter().all(|pos| board.check((pos.x, pos.y - 1)));
    if !can_move_down {
        if lockdown_timer.finished() {
            lockdown_timer.reset();
        }
        return;
    }
    if soft_drop.active {
        score.score += 1;
    }
    // If the lockdown timer is running,
    // but we have space to fall, we should cancel this timer,
    // by supplying it with ridiculously big value.
    if !lockdown_timer.finished() {
        lockdown_timer.tick(Duration::from_secs(10));
    }
    for mut pos in tiles.iter_mut() {
        if !board.check((pos.x, pos.y - 1)) {
            continue;
        }

        pos.y -= 1;
    }
}

pub fn clear_lines(
    mut board: ResMut<GameBoard>,
    lockdown_timer: Res<LockdownTimer>,
    mut score: ResMut<Score>,
) {
    // We only check for lines when the lockdown timer is
    // finished. Because we don't want to clear lines while
    // the tiles are moving down.
    if !lockdown_timer.finished() {
        return;
    }
    let cleared = board.clear_lines();
    if cleared > 0 {
        let mut score_delta = 100;
        for _ in 1..cleared {
            score_delta += 200;
        }
        score.score += score_delta;
        score.lines_cleared += cleared;
    }
}
