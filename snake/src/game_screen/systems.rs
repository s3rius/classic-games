use bevy::{prelude::*, utils::HashSet};
use rand::Rng;

use crate::{
    consts::{self, BLOCK_SIZE, FOREGROUND_COL},
    state::{GameState, Score},
};

use super::{
    components::{Direction, Food, SnakeHead, SnakePart, SnakeTail, VirtualPosition},
    events::{EatingEvent, GameOverEvent},
    resources::MainTimer,
};

pub fn reset_score(mut score: ResMut<Score>) {
    score.0 = 0;
}

pub fn reset_timer(mut timer: ResMut<MainTimer>) {
    timer.reset();
}

pub fn spawn_snake(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: FOREGROUND_COL,
                custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                ..default()
            },
            ..default()
        },
        VirtualPosition { x: 0, y: 0 },
        SnakePart,
        SnakeHead {
            direction: Direction::Right,
            locked: false,
        },
    ));
    commands.spawn((VirtualPosition { x: -1, y: 0 }, SnakeTail));
}

pub fn spawn_food(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: FOREGROUND_COL,
                custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                ..default()
            },
            ..default()
        },
        VirtualPosition { x: 5, y: 5 },
        Food,
    ));
}

pub fn move_snake(
    time: Res<Time>,
    mut timer: ResMut<MainTimer>,
    mut head: Query<(&mut VirtualPosition, &mut SnakeHead)>,
    mut snake_parts_query: Query<
        (Entity, &mut VirtualPosition, &mut Visibility),
        (With<SnakePart>, Without<SnakeHead>),
    >,
    mut tail_query: Query<
        &mut VirtualPosition,
        (With<SnakeTail>, Without<SnakePart>, Without<SnakeHead>),
    >,
) {
    // If timer hasn't finished yet.
    if !timer.tick(time.delta()).finished() {
        return;
    }
    let mut tail_pos = tail_query.single_mut();
    let (mut head_position, mut head) = head.single_mut();
    let mut last_pos = head_position.clone();
    let new_position = match head.direction {
        Direction::Up => VirtualPosition {
            x: head_position.x,
            y: head_position.y + 1,
        },
        Direction::Down => VirtualPosition {
            x: head_position.x,
            y: head_position.y - 1,
        },
        Direction::Left => VirtualPosition {
            x: head_position.x - 1,
            y: head_position.y,
        },
        Direction::Right => VirtualPosition {
            x: head_position.x + 1,
            y: head_position.y,
        },
    };
    let snake_parts_sorted = snake_parts_query.iter_mut().sort::<Entity>();
    for (_, mut position, mut visibility) in snake_parts_sorted {
        let tmp = *position;
        *position = last_pos;
        last_pos = tmp;
        *visibility = Visibility::Visible;
    }
    *tail_pos = last_pos;
    *head_position = new_position;
    if head.locked {
        head.locked = false;
    }
}

pub fn check_on_food(
    head_query: Query<&VirtualPosition, With<SnakeHead>>,
    food_query: Query<&VirtualPosition, With<Food>>,
    mut eating_event_chan: EventWriter<EatingEvent>,
) {
    let head_pos = head_query.single();
    let food_pos = food_query.single();
    if head_pos == food_pos {
        eating_event_chan.send(EatingEvent);
    }
}

pub fn check_on_cannibalism(
    head_query: Query<&VirtualPosition, With<SnakeHead>>,
    snake_parts: Query<&VirtualPosition, (With<SnakePart>, Without<SnakeHead>)>,
    mut gover_event: EventWriter<GameOverEvent>,
) {
    let head_pos = head_query.single();
    for part_pos in &mut snake_parts.iter() {
        if head_pos == part_pos {
            gover_event.send(GameOverEvent);
        }
    }
}

pub fn check_on_edge_running(
    camera_query: Query<&Camera>,
    head_query: Query<&Transform, With<SnakeHead>>,
    mut gover_event: EventWriter<GameOverEvent>,
) {
    let camera = camera_query.single();
    // A window size.
    let Some(view_rect) = camera.physical_viewport_rect() else {
        return;
    };
    let head_transform = head_query.single();
    // Player reached the edge of the screen on x axis.
    if view_rect.max.x as f32 / 2. <= head_transform.translation.x.abs() {
        gover_event.send(GameOverEvent);
    }
    // Player reached the edge of the screen on y axis.
    if view_rect.max.y as f32 / 2. <= head_transform.translation.y.abs() {
        gover_event.send(GameOverEvent);
    }
}

pub fn eat_food(
    mut food_pos_query: Query<(&mut VirtualPosition, &mut Visibility), With<Food>>,
    snake_parts: Query<&VirtualPosition, (With<SnakePart>, Without<Food>)>,
    camera_query: Query<&Camera>,
    mut score: ResMut<Score>,
) {
    let camera = camera_query.single();
    let (mut food_position, mut visibility) = food_pos_query.single_mut();
    *visibility = Visibility::Hidden;
    let mut pos = HashSet::<(i32, i32)>::new();
    for snake_part_pos in &mut snake_parts.iter() {
        pos.insert((snake_part_pos.x, snake_part_pos.y));
    }
    let Some(view_rect) = camera.physical_viewport_rect() else {
        return;
    };
    // This expression calculates the number of blocks that can fit in the window.
    let x_blocks = view_rect.max.x as f32 / BLOCK_SIZE - BLOCK_SIZE;
    let y_blocks = view_rect.max.y as f32 / BLOCK_SIZE - BLOCK_SIZE;
    // We divide by 2 to get the number of blocks that can fit in half of the window.
    let virt_x_max = (x_blocks / 2.) as i32;
    let virt_y_max = (y_blocks / 2.) as i32;
    let mut rng = rand::thread_rng();
    loop {
        let x: i32 = rng.gen_range(-virt_x_max..virt_x_max);
        let y: i32 = rng.gen_range(-virt_y_max..virt_y_max);
        if !pos.contains(&(x, y)) {
            *food_position = VirtualPosition { x, y };
            break;
        }
    }
    *visibility = Visibility::Visible;
    score.0 += 1;
}

pub fn grow_snake(camera_query: Query<&Camera>, mut commands: Commands) {
    let camera = camera_query.single();
    let Some(view_rect) = camera.physical_viewport_rect() else {
        return;
    };
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: FOREGROUND_COL,
                custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                ..default()
            },
            visibility: Visibility::Hidden,
            transform: Transform::from_translation(Vec3::new(
                view_rect.max.x as f32 * 2.,
                0.0,
                0.0,
            )),
            ..default()
        },
        VirtualPosition {
            x: view_rect.max.x as i32,
            y: view_rect.max.y as i32,
        },
        SnakePart,
    ));
}

pub fn game_over(mut app_state: ResMut<NextState<GameState>>) {
    app_state.set(GameState::GameOver);
}

pub fn handle_input(keys: Res<ButtonInput<KeyCode>>, mut head_query: Query<&mut SnakeHead>) {
    let mut head = head_query.single_mut();
    let mut new_direction = head.direction;
    if keys.just_pressed(KeyCode::ArrowUp) && head.direction != Direction::Down {
        new_direction = Direction::Up;
    }
    if keys.just_pressed(KeyCode::ArrowDown) && head.direction != Direction::Up {
        new_direction = Direction::Down;
    }
    if keys.just_pressed(KeyCode::ArrowRight) && head.direction != Direction::Left {
        new_direction = Direction::Right;
    }
    if keys.just_pressed(KeyCode::ArrowLeft) && head.direction != Direction::Right {
        new_direction = Direction::Left;
    }
    if !head.locked && head.direction != new_direction {
        head.direction = new_direction;
        head.locked = true;
    }
}

pub fn despawn_snake(
    head_query: Query<Entity, With<SnakeHead>>,
    snake_query: Query<Entity, With<SnakePart>>,
    tail_query: Query<Entity, With<SnakeTail>>,
    food_query: Query<Entity, With<Food>>,
    mut commands: Commands,
) {
    for entity in &head_query {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &snake_query {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &tail_query {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &food_query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn position_translation(mut q: Query<(&VirtualPosition, &mut Transform)>) {
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            pos.x as f32 * consts::BLOCK_SIZE,
            pos.y as f32 * consts::BLOCK_SIZE,
            0.0,
        );
    }
}
