use bevy::{prelude::*, utils::HashMap};

use crate::{
    consts,
    globals::{GameDifficulty, GameState, SoltutionTimer},
    utils::{
        components::{Focusable, HasFocus},
        direction::Direction,
        maze::Maze,
    },
};

use super::{
    componenets::{GridPosition, PipeSprite},
    events::{ChangeFocusEvent, PipeCompletedEvent, RotatePipeEvent},
    game_board::{GameBoard, PipeType},
};

pub fn restart_timer(mut timer: ResMut<SoltutionTimer>) {
    timer.reset();
    timer.unpause();
}

pub fn generate_game_board(mut board: ResMut<GameBoard>, difficulty: Res<GameDifficulty>) {
    let maze = Maze::builder(difficulty.size)
        .with_avoid_straight(85)
        .with_wrap(difficulty.wrap)
        .build();
    board.generate(&maze);
}

pub fn setup_camera(
    focused_query: Query<&Transform, With<HasFocus>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<HasFocus>)>,
) {
    let mut camera_transform = camera_query.single_mut();
    let focus_target = focused_query.single();
    camera_transform.translation = focus_target.translation;
}

pub fn render_grid(mut commands: Commands, asset_server: Res<AssetServer>, board: Res<GameBoard>) {
    let center = board.grid.len() / 2;
    for (y, row) in board.grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let texture = match cell.pipe_type {
                PipeType::DeadEnd => consts::PIPE_DEAD_END_TEXTURE,
                PipeType::Straight => consts::PIPE_STRAIGH_TEXTURE,
                PipeType::Elbow => consts::PIPE_ELBOW_TEXTURE,
                PipeType::Tee => consts::PIPE_TEE_TEXTURE,
                PipeType::Cross => consts::PIPE_CROSS_TEXTURE,
            };
            let mut entity = commands.spawn((
                SpriteBundle {
                    texture: asset_server.load(texture),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(consts::CELL_SIZE, consts::CELL_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        x as f32 * consts::CELL_SIZE,
                        (board.grid.len() - y - 1) as f32 * consts::CELL_SIZE,
                        0.,
                    )),
                    ..Default::default()
                },
                GridPosition { x, y },
                Focusable,
                PipeSprite,
            ));
            if x == center && y == center {
                entity.insert(HasFocus);
            }
        }
    }
}

pub fn camera_follow_cursor(
    focused_query: Query<&Transform, With<HasFocus>>,
    mut camera_query: Query<(&Camera, &mut Transform), Without<HasFocus>>,
) {
    let focus_target = focused_query.single();
    let (camera, mut camera_transform) = camera_query.single_mut();
    let Some(rect) = camera.logical_viewport_size() else {
        return;
    };
    let gap = consts::CELL_SIZE / 2.;
    if focus_target.translation.x + gap > camera_transform.translation.x + rect.x / 2. {
        camera_transform.translation.x += consts::CELL_SIZE;
    }
    if focus_target.translation.x - gap < camera_transform.translation.x - rect.x / 2. {
        camera_transform.translation.x -= consts::CELL_SIZE;
    }
    if focus_target.translation.y + gap > camera_transform.translation.y + rect.y / 2. {
        camera_transform.translation.y += consts::CELL_SIZE;
    }
    if focus_target.translation.y - gap < camera_transform.translation.y - rect.y / 2. {
        camera_transform.translation.y -= consts::CELL_SIZE;
    }
}

pub fn highlight_connected(
    board: Res<GameBoard>,
    mut pipe_parts_query: Query<(&GridPosition, &mut Sprite), With<PipeSprite>>,
) {
    for (position, mut sprite) in pipe_parts_query.iter_mut() {
        if board.connected.contains(&(position.x, position.y)) {
            sprite.color = consts::CONNECTED_PIPE_COLOR;
        } else {
            sprite.color = consts::PIPE_COLOR;
        }
    }
}

pub fn highlight_focused(
    focused_query: Query<&GridPosition, With<HasFocus>>,
    mut pipe_parts_query: Query<(&GridPosition, &mut Sprite), With<PipeSprite>>,
) {
    let focus_target = focused_query.single();
    for (position, mut sprite) in pipe_parts_query.iter_mut() {
        if position == focus_target {
            sprite.color = consts::FOCUSED_PIPE_COLOR
        }
    }
}

pub fn rotate_focused(
    mut board: ResMut<GameBoard>,
    mut focused: Query<&GridPosition, With<HasFocus>>,
) {
    let focused_pos = focused.single_mut();
    board.rotate(focused_pos.x, focused_pos.y);
}

pub fn fix_rotations(
    board: Res<GameBoard>,
    mut pipes: Query<(&mut Transform, &GridPosition), With<PipeSprite>>,
    time: Res<Time>,
) {
    for (mut transform, position) in pipes.iter_mut() {
        let cell = &board.grid[position.y][position.x];
        transform.rotation = transform
            .rotation
            .slerp(cell.rotation.to_quat(), time.delta().as_secs_f32() * 8.5);
    }
}

pub fn check_on_completed(
    board: Res<GameBoard>,
    mut state: ResMut<NextState<GameState>>,
    mut pipe_completed: EventWriter<PipeCompletedEvent>,
    mut timer: ResMut<SoltutionTimer>,
    time: Res<Time>,
) {
    if timer.paused() {
        return;
    }
    timer.tick(time.delta());
    if board.is_solved() {
        timer.pause();
        pipe_completed.send_default();
        state.set(GameState::GameOver);
    }
}

pub fn change_focus(
    mut event_reader: EventReader<ChangeFocusEvent>,
    mut commands: Commands,
    pipe_parts_query: Query<(Entity, &GridPosition, Option<&HasFocus>), With<Focusable>>,
    timer: ResMut<SoltutionTimer>,
) {
    if timer.paused() {
        return;
    }
    let (mut offset_x, mut offset_y) = (0, 0);
    // Here we iterate over all the ChangeFocusEvent events that have been sent
    // and we update the offset_x and offset_y values based on the direction of all events.
    for ChangeFocusEvent(focus_direction) in event_reader.read() {
        let (dx, dy) = focus_direction.offset();
        offset_x += dx;
        offset_y += dy;
    }
    let mut pipe_map = HashMap::new();
    // No particular reason to start from this position, just a random value.
    let mut current_focus_pos = GridPosition { x: 3, y: 3 };
    let mut max_x = 0;
    let mut max_y = 0;
    for (pipe_part_en, position, has_focus) in pipe_parts_query.iter() {
        if has_focus.is_some() {
            current_focus_pos = *position;
            commands.entity(pipe_part_en).remove::<HasFocus>();
        }
        if position.x > max_x {
            max_x = position.x;
        }
        if position.y > max_y {
            max_y = position.y;
        }
        pipe_map.insert(*position, pipe_part_en);
    }
    let mut next_pos = GridPosition {
        x: usize::try_from(current_focus_pos.x as i32 + offset_x).unwrap_or(max_x),
        y: usize::try_from(current_focus_pos.y as i32 + offset_y).unwrap_or(max_y),
    };
    if next_pos.x > max_x {
        next_pos.x = 0;
    }
    if next_pos.y > max_y {
        next_pos.y = 0;
    }
    if let Some(pipe_part_en) = pipe_map.get(&next_pos) {
        commands.entity(*pipe_part_en).insert(HasFocus);
    }
}

pub fn controls(
    key: Res<ButtonInput<KeyCode>>,
    mut change_focus_writer: EventWriter<ChangeFocusEvent>,
    mut rotate_writer: EventWriter<RotatePipeEvent>,
) {
    if key.just_pressed(KeyCode::ArrowUp) {
        change_focus_writer.send(ChangeFocusEvent(Direction::Up));
    }
    if key.just_pressed(KeyCode::ArrowDown) {
        change_focus_writer.send(ChangeFocusEvent(Direction::Down));
    }
    if key.just_pressed(KeyCode::ArrowLeft) {
        change_focus_writer.send(ChangeFocusEvent(Direction::Left));
    }
    if key.just_pressed(KeyCode::ArrowRight) {
        change_focus_writer.send(ChangeFocusEvent(Direction::Right));
    }
    if key.just_pressed(KeyCode::Space) || key.just_pressed(KeyCode::Enter) {
        rotate_writer.send(RotatePipeEvent);
    }
}
