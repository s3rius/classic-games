use bevy::prelude::*;

use crate::{
    consts,
    globals::{GameDifficulty, GameState},
    utils::components::{Focusable, HasFocus},
};

use super::{
    components::{OnSelectDifficulty, OnSelectSize, OnSelectWrap, SelectSize, SelectWrap},
    states::DifficultySelectState,
};

pub fn init_substate(mut substate: ResMut<NextState<DifficultySelectState>>) {
    substate.set(DifficultySelectState::SelectingSize);
}

pub fn setup_size_select(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_style = TextStyle {
        font_size: 16.,
        font: asset_server.load(consts::FONT_RESOURCE),
        color: consts::FOREGROUND_COLOR,
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
            OnSelectDifficulty,
            OnSelectSize,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section("Select size", text_style.clone()),
                ..default()
            });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        row_gap: Val::Percent(5.),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    for size in [5, 7, 9, 11, 13] {
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
                                SelectSize(size),
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle {
                                    text: Text::from_section(
                                        format!("{size}x{size}"),
                                        text_style.clone(),
                                    ),
                                    ..default()
                                });
                            });
                    }
                });
        });
}

pub fn setup_wrap_select(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_style = TextStyle {
        font_size: 16.,
        font: asset_server.load(consts::FONT_RESOURCE),
        color: consts::FOREGROUND_COLOR,
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
            OnSelectDifficulty,
            OnSelectWrap,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section("Wrap grid?", text_style.clone()),
                ..default()
            });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        row_gap: Val::Percent(5.),
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
                            Focusable,
                            SelectWrap(false),
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section(format!("NO"), text_style.clone()),
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
                            SelectWrap(true),
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section(format!("YES"), text_style.clone()),
                                ..default()
                            });
                        });
                });
        });
}

pub fn handle_select_size(
    inputs: Res<ButtonInput<KeyCode>>,
    focued_button_query: Query<&SelectSize, With<HasFocus>>,
    mut difficulty: ResMut<GameDifficulty>,
    mut menu_state: ResMut<NextState<DifficultySelectState>>,
) {
    if inputs.just_pressed(KeyCode::Enter) {
        let size = focued_button_query.single();
        difficulty.size = size.0;
        menu_state.set(DifficultySelectState::SelectingWrap);
    }
}

pub fn handle_select_wrap(
    inputs: Res<ButtonInput<KeyCode>>,
    focued_button_query: Query<&SelectWrap, With<HasFocus>>,
    mut difficulty: ResMut<GameDifficulty>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if inputs.just_pressed(KeyCode::Enter) {
        let wrap = focued_button_query.single();
        difficulty.wrap = wrap.0;
        game_state.set(GameState::Playing);
    }
}

pub fn focused_button_decoration(
    mut buttons: Query<(&mut BorderColor, Option<&HasFocus>), With<Focusable>>,
) {
    for (mut style, focus) in buttons.iter_mut() {
        if focus.is_some() {
            *style = BorderColor(consts::FOREGROUND_COLOR);
        } else {
            *style = BorderColor(Color::NONE);
        }
    }
}
