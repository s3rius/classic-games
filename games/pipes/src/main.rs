#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[allow(unused)]
use bevy::{prelude::*, time::Stopwatch, window::WindowResolution};

pub mod assets;
pub mod consts;
pub mod difficulty_select;
pub mod game_over;
pub mod game_screen;
pub mod globals;
pub mod start_menu;
pub mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            #[cfg(not(target_arch = "wasm32"))]
            primary_window: Some(Window {
                title: String::from("Testris"),
                mode: bevy::window::WindowMode::Windowed,
                resolution: WindowResolution::new(1280., 720.),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                ..default()
            }),
            #[cfg(target_arch = "wasm32")]
            primary_window: Some(Window {
                canvas: Some(String::from("#gameboard")),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_camera)
        .add_systems(Update, global_controls)
        .init_state::<globals::GameState>()
        .insert_resource(globals::GameDifficulty::default())
        .insert_resource(globals::SoltutionTimer(Stopwatch::new()))
        .add_plugins(assets::AssetsPlugin)
        .add_plugins(start_menu::StartMenu)
        .add_plugins(difficulty_select::DifficultySelect)
        .add_plugins(game_screen::GameScreenPlugin)
        .add_plugins(game_over::GameOverPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            clear_color: ClearColorConfig::Custom(consts::BACKGROUND_COLOR),
            hdr: true,
            ..default()
        },
        ..default()
    });
}

#[allow(unused)]
fn global_controls(keys: Res<ButtonInput<KeyCode>>, mut exit_writer: EventWriter<AppExit>) {
    #[cfg(not(target_arch = "wasm32"))]
    if keys.just_pressed(KeyCode::KeyQ) {
        exit_writer.send(AppExit::Success);
    }
}
