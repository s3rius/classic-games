use bevy::prelude::*;
use cgc_lib::simple_menu::SimpleMenuButton;

use crate::{consts, globals::GameState};

pub struct StartMenu;

impl Plugin for StartMenu {
    fn build(&self, app: &mut App) {
        let asset_server = app.world().resource::<AssetServer>();
        let default_text_style = TextStyle {
            font: asset_server.load(cgc_lib::COMMON_FONT_NAME),
            font_size: 16.,
            color: consts::FOREGROUND_COLOR,
        };
        app.add_event::<ButtonAction>()
            .add_plugins(cgc_lib::simple_menu::SimpleMenuPlugin {
                on_state: GameState::StartMenu,
                btn_color: consts::BACKGROUND_COLOR,
                border_color: consts::FOREGROUND_COLOR,
                buttons_row_gap: Val::Percent(20.),
                content_row_gap: Val::Percent(15.),
                buttons: vec![
                    SimpleMenuButton {
                        text: "START".to_string(),
                        event: ButtonAction::StartGame,
                        text_style: default_text_style.clone(),
                    },
                    SimpleMenuButton {
                        text: "EXIT".to_string(),
                        event: ButtonAction::StartGame,
                        text_style: default_text_style.clone(),
                    },
                ],
                extra_children: Some(move |builder: &mut ChildBuilder| {
                    builder.spawn(TextBundle {
                        text: Text::from_section(
                            "PIPES",
                            TextStyle {
                                font_size: 32.,
                                ..default_text_style.clone()
                            },
                        ),
                        ..default()
                    });
                }),
            })
            .add_systems(Update, on_button_event.run_if(on_event::<ButtonAction>()));
    }
}

#[derive(Clone, Debug, Event)]
pub enum ButtonAction {
    StartGame,
    Exit,
}

pub fn on_button_event(
    mut event_reader: EventReader<ButtonAction>,
    mut app_exit: EventWriter<AppExit>,
    mut app_state: ResMut<NextState<GameState>>,
) {
    for event in event_reader.read() {
        match event {
            ButtonAction::StartGame => {
                app_state.set(GameState::SelectDifficulty);
            }
            ButtonAction::Exit => {
                app_exit.send(AppExit::Success);
            }
        }
    }
}
