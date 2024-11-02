use bevy::prelude::*;

use crate::{
    consts,
    globals::{GameDifficulty, GameState},
};

pub struct DifficultySelect;

impl Plugin for DifficultySelect {
    fn build(&self, app: &mut App) {
        let asset_server = app.world().resource::<AssetServer>();
        let default_text_style = TextStyle {
            font: asset_server.load(cgc_lib::COMMON_FONT_NAME),
            font_size: 16.,
            color: consts::FOREGROUND_COLOR,
        };

        let mut size_buttons = vec![];
        for size in [5, 7, 9, 11, 13] {
            size_buttons.push(cgc_lib::simple_menu::SimpleMenuButton {
                text: format!("{size}x{size}"),
                event: SelectSizeEvent(size),
                text_style: default_text_style.clone(),
            });
        }
        // Required to move it to closure.
        let cloned_text_style = default_text_style.clone();

        app.add_event::<SelectSizeEvent>()
            .add_event::<SelectWrapEvent>()
            .add_sub_state::<DifficultySelectState>()
            .add_systems(OnEnter(GameState::SelectDifficulty), init_substate)
            .add_plugins(cgc_lib::simple_menu::SimpleMenuPlugin {
                on_state: DifficultySelectState::SelectingSize,
                btn_color: consts::FOREGROUND_COLOR,
                border_color: consts::FOREGROUND_COLOR,
                content_row_gap: Val::Percent(15.),
                buttons_row_gap: Val::Percent(5.),
                buttons: size_buttons,
                extra_children: Some(move |parent: &mut ChildBuilder| {
                    parent.spawn(TextBundle {
                        text: Text::from_section("SELECT SIZE", default_text_style.clone()),
                        ..default()
                    });
                }),
            })
            .add_plugins(cgc_lib::simple_menu::SimpleMenuPlugin {
                on_state: DifficultySelectState::SelectingWrap,
                btn_color: consts::FOREGROUND_COLOR,
                border_color: consts::FOREGROUND_COLOR,
                content_row_gap: Val::Percent(15.),
                buttons_row_gap: Val::Percent(5.),
                buttons: vec![
                    cgc_lib::simple_menu::SimpleMenuButton {
                        text: String::from("NO"),
                        event: SelectWrapEvent(false),
                        text_style: cloned_text_style.clone(),
                    },
                    cgc_lib::simple_menu::SimpleMenuButton {
                        text: String::from("YES"),
                        event: SelectWrapEvent(true),
                        text_style: cloned_text_style.clone(),
                    },
                ],
                extra_children: Some(move |parent: &mut ChildBuilder| {
                    parent.spawn(TextBundle {
                        text: Text::from_section("wrap grid?", cloned_text_style.clone()),
                        ..default()
                    });
                }),
            })
            .add_systems(
                Update,
                (on_select_size, on_select_wrap)
                    .distributive_run_if(in_state(GameState::SelectDifficulty)),
            );
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, SubStates)]
#[source(GameState = GameState::SelectDifficulty)]
pub enum DifficultySelectState {
    #[default]
    SelectingSize,
    SelectingWrap,
}

#[derive(Debug, Clone, Default, Event)]
pub struct SelectSizeEvent(pub usize);

#[derive(Debug, Clone, Default, Event)]
pub struct SelectWrapEvent(pub bool);

/// Size was selected.
///
/// We update global difficulty variable
/// and move to the next menu.
pub fn on_select_size(
    mut event_reader: EventReader<SelectSizeEvent>,
    mut difficulty: ResMut<GameDifficulty>,
    mut menu_state: ResMut<NextState<DifficultySelectState>>,
) {
    for event in event_reader.read() {
        difficulty.size = event.0;
        menu_state.set(DifficultySelectState::SelectingWrap);
    }
}

/// Wrap setting was set.
///
/// We update global difficulty variable
/// and move to the game screen.
pub fn on_select_wrap(
    mut event_reader: EventReader<SelectWrapEvent>,
    mut difficulty: ResMut<GameDifficulty>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for event in event_reader.read() {
        difficulty.wrap = event.0;
        game_state.set(GameState::Playing);
    }
}

/// Initialize the substate to `SelectingSize`.
///
/// This substate indicates in which exact menu we are currently.
pub fn init_substate(mut substate: ResMut<NextState<DifficultySelectState>>) {
    substate.set(DifficultySelectState::SelectingSize);
}
