use bevy::prelude::*;

use crate::{
    consts,
    globals::{GameDifficulty, GameState, SoltutionTimer},
    utils::components::{Focusable, HasFocus},
};

use super::components::{ButtonAction, OnGameOverScreen};
use cgc_lib::COMMON_FONT_NAME;

pub fn render_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    difficulty: Res<GameDifficulty>,
    timer: Res<SoltutionTimer>,
) {
    let text_style = TextStyle {
        font_size: 16.,
        font: asset_server.load(COMMON_FONT_NAME),
        color: consts::FOREGROUND_COLOR,
        ..default()
    };
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    top: Val::Px(0.),
                    bottom: Val::Px(0.),
                    row_gap: Val::Percent(5.),
                    flex_direction: FlexDirection::Column,
                    align_self: AlignSelf::Center,
                    align_content: AlignContent::Center,
                    justify_self: JustifySelf::Center,
                    ..default()
                },
                ..default()
            },
            OnGameOverScreen,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Puzzle completed!",
                    TextStyle {
                        font_size: 32.,
                        ..text_style.clone()
                    },
                ),
                ..default()
            });
            parent.spawn(TextBundle {
                text: Text::from_section(
                    format!("Size: {}x{}", difficulty.size, difficulty.size),
                    text_style.clone(),
                ),
                ..default()
            });
            parent.spawn(TextBundle {
                text: Text::from_section(
                    format!("Took {:.2} seconds", timer.elapsed_secs()),
                    text_style.clone(),
                ),
                ..default()
            });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        row_gap: Val::Percent(20.),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    border: UiRect::all(Val::Px(3.)),
                                    padding: UiRect::all(Val::Percent(4.)),
                                    ..default()
                                },
                                ..default()
                            },
                            HasFocus,
                            Focusable,
                            ButtonAction::Restart,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section("RESTART", text_style.clone()),
                                ..default()
                            });
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    border: UiRect::all(Val::Px(3.)),
                                    padding: UiRect::all(Val::Percent(4.)),
                                    ..default()
                                },
                                ..default()
                            },
                            Focusable,
                            ButtonAction::ChangeDifficulty,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section("CHANGE DIFFICULTY", text_style.clone()),
                                ..default()
                            });
                        });
                    #[cfg(not(target_arch = "wasm32"))]
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    border: UiRect::all(Val::Px(3.)),
                                    padding: UiRect::all(Val::Percent(4.)),
                                    ..default()
                                },
                                ..default()
                            },
                            Focusable,
                            ButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section("QUIT", text_style.clone()),
                                ..default()
                            });
                        });
                });
        });
}

pub fn focused_button_decoration(
    mut buttons: Query<(&mut BorderColor, Option<&HasFocus>), With<Button>>,
) {
    for (mut style, focus) in buttons.iter_mut() {
        if focus.is_some() {
            *style = BorderColor(consts::FOREGROUND_COLOR);
        } else {
            *style = BorderColor(Color::NONE);
        }
    }
}

pub fn on_select_item(
    inputs: Res<ButtonInput<KeyCode>>,
    focued_button_query: Query<&ButtonAction, With<HasFocus>>,
    mut app_exit: EventWriter<AppExit>,
    mut app_state: ResMut<NextState<GameState>>,
) {
    if inputs.just_pressed(KeyCode::Enter) {
        match focued_button_query.single() {
            ButtonAction::Quit => {
                app_exit.send(AppExit::Success);
            }
            ButtonAction::Restart => {
                app_state.set(GameState::Playing);
            }
            ButtonAction::ChangeDifficulty => {
                app_state.set(GameState::SelectDifficulty);
            }
        }
    }
}
