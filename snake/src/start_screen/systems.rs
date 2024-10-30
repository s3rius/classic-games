use bevy::prelude::*;

use crate::{consts, state::GameState, utils::components::HasFocus};

use super::components::{ButtonAction, OnMenuScreen};

pub fn render_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_style = TextStyle {
        font_size: 16.,
        font: asset_server.load(cgc_lib::COMMON_FONT_NAME),
        color: consts::FOREGROUND_COL,
        ..default()
    };
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    top: Val::Px(0.),
                    bottom: Val::Px(0.),
                    row_gap: Val::Percent(15.),
                    flex_direction: FlexDirection::Column,
                    align_self: AlignSelf::Center,
                    align_content: AlignContent::Center,
                    justify_self: JustifySelf::Center,
                    ..default()
                },
                ..default()
            },
            OnMenuScreen,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "_SNAKE_",
                    TextStyle {
                        font_size: 32.,
                        ..text_style.clone()
                    },
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
                            ButtonAction::StartGame,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section("START", text_style.clone()),
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
            *style = BorderColor(consts::FOREGROUND_COL);
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
            ButtonAction::StartGame => {
                app_state.set(GameState::Playing);
            }
        }
    }
}
