#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::{prelude::*, window::WindowResolution};

mod consts;
mod death_screen;
mod game_screen;
mod start_screen;
mod state;
mod utils;

fn main() {
    bevy::app::App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(800., 600.).with_scale_factor_override(1.),
                title: String::from("Snake"),
                ..default()
            }),
            ..default()
        }))
        .init_state::<state::GameState>()
        .insert_resource(state::Score(0))
        .add_systems(Startup, setup_camera)
        .add_systems(Update, exit_game)
        .add_plugins(cgc_lib::CGCCommonAssetsPlugin)
        .add_plugins(start_screen::StartScreenPlugin)
        .add_plugins(game_screen::GameScreenPlugin)
        .add_plugins(death_screen::DeathScreenPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            hdr: true,
            clear_color: ClearColorConfig::Custom(consts::BACKGROUND_COL),
            ..default()
        },
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
}

fn exit_game(keys: Res<ButtonInput<KeyCode>>, mut exit_writer: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::KeyQ) {
        exit_writer.send(AppExit::Success);
    }
}
