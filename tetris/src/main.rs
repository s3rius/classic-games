#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::{prelude::*, window::WindowResolution};

pub mod assets;
pub mod consts;
pub mod game_screen;
pub mod gameover_screen;
pub mod start_menu;
pub mod state;
pub mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Testris"),
                mode: bevy::window::WindowMode::Windowed,
                resolution: WindowResolution::new(1280., 720.),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_camera)
        .add_systems(Update, exit_game)
        .insert_resource(state::Score::default())
        .insert_resource(state::Debug::default())
        .init_state::<state::GameState>()
        .add_plugins(assets::TetrisAssetsPlugin)
        .add_plugins(start_menu::StartMenu)
        .add_plugins(game_screen::GameScreenPlugin)
        .add_plugins(gameover_screen::GameoverScreenPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            clear_color: ClearColorConfig::Custom(consts::BACKGROUND_COLOR),
            hdr: true,
            ..default()
        },
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
}

#[allow(unused)]
fn exit_game(
    keys: Res<ButtonInput<KeyCode>>,
    mut exit_writer: EventWriter<AppExit>,
    mut debug_res: ResMut<state::Debug>,
) {
    if keys.just_pressed(KeyCode::KeyQ) {
        exit_writer.send(AppExit::Success);
    }
    #[cfg(debug_assertions)]
    if keys.just_pressed(KeyCode::KeyD) {
        debug_res.enabled = !debug_res.enabled;
    }
}
