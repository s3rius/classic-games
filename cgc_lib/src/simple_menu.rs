use bevy::prelude::*;

use crate::despawn_entities;

#[derive(Clone)]
pub struct SimpleMenuPlugin<T, EC, BE>
where
    T: States,
    EC: FnOnce(&mut ChildBuilder) + Send + Sync + Clone,
    BE: Event + Clone,
{
    pub on_state: T,
    pub btn_color: Color,
    pub border_color: Color,
    pub content_row_gap: Val,
    pub buttons_row_gap: Val,
    pub extra_children: Option<EC>,
    pub buttons: Vec<SimpleMenuButton<BE>>,
}

impl<T, EC, BE> Plugin for SimpleMenuPlugin<T, EC, BE>
where
    T: States,
    EC: FnOnce(&mut ChildBuilder) + Clone + Sync + Send + Clone + 'static,
    BE: Event + Clone,
{
    fn build(&self, app: &mut App) {
        let cloned = self.clone();
        app.add_systems(
            OnEnter(self.on_state.clone()),
            (
                move |commands: Commands| {
                    render_menu::<BE>(
                        commands,
                        cloned.content_row_gap.clone(),
                        cloned.buttons_row_gap.clone(),
                        cloned.buttons.clone(),
                        cloned.extra_children.clone(),
                    );
                },
                focus_first,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                vert_menu_controls,
                on_select_item::<BE>,
                move |buttons: Query<(&mut BorderColor, Option<&HasFocus>), With<Focusable>>| {
                    focused_button_decoration(buttons, cloned.border_color.clone())
                },
            )
                .distributive_run_if(in_state(self.on_state.clone())),
        )
        .add_systems(
            OnExit(self.on_state.clone()),
            despawn_entities::<OnMenuScreen>,
        );
    }
}

impl<S, EC, BE> Default for SimpleMenuPlugin<S, EC, BE>
where
    S: States + Default,
    EC: FnOnce(&mut ChildBuilder) + Send + Sync + Clone,
    BE: Event + Clone,
{
    fn default() -> Self {
        Self {
            on_state: S::default(),
            btn_color: Color::BLACK,
            border_color: Color::BLACK,
            content_row_gap: Val::Percent(15.),
            buttons_row_gap: Val::Percent(5.),
            extra_children: None,
            buttons: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct SimpleMenuButton<T: Event> {
    pub text: String,
    pub event: T,
    pub text_style: TextStyle,
}

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct OnClick<T: Event>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct Focusable;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct HasFocus;

#[derive(Debug, Component)]
pub struct OnMenuScreen;

pub fn render_menu<T: Event>(
    mut commands: Commands,
    content_row_gap: Val,
    buttons_row_gap: Val,
    buttons: Vec<SimpleMenuButton<T>>,
    extra_children: Option<impl FnOnce(&mut ChildBuilder)>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    top: Val::Px(0.),
                    bottom: Val::Px(0.),
                    row_gap: content_row_gap,
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
            parent
                .spawn(NodeBundle {
                    style: Style {
                        row_gap: buttons_row_gap,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    if let Some(extra_children) = extra_children {
                        extra_children(parent);
                    }
                    for button in buttons {
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
                                OnClick(button.event),
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle {
                                    text: Text::from_section(button.text, button.text_style),
                                    ..default()
                                });
                            });
                    }
                });
        });
}

pub fn focused_button_decoration(
    mut buttons: Query<(&mut BorderColor, Option<&HasFocus>), With<Focusable>>,
    border_color: Color,
) {
    for (mut style, focus) in buttons.iter_mut() {
        if focus.is_some() {
            *style = BorderColor(border_color);
        } else {
            *style = BorderColor(Color::NONE);
        }
    }
}

pub fn on_select_item<E: Event + Clone>(
    inputs: Res<ButtonInput<KeyCode>>,
    focued_button_query: Query<&OnClick<E>, With<HasFocus>>,
    mut event_writer: EventWriter<E>,
) {
    if inputs.just_pressed(KeyCode::Enter) {
        if let Some(event) = focued_button_query.iter().next() {
            event_writer.send(event.0.clone());
        }
    }
}

pub fn focus_first(
    elements: Query<(Entity, Option<&HasFocus>, &Transform), With<Focusable>>,
    mut commands: Commands,
) {
    let a = elements
        .iter()
        .sort_by::<&Transform>(|a, b| a.translation.y.total_cmp(&b.translation.y))
        .collect::<Vec<_>>();
    if a.is_empty() {
        return;
    }
    for (entity, focus, _) in &a {
        if focus.is_some() {
            commands.entity(*entity).remove::<HasFocus>();
        }
    }
    if let Some((entity, _, _)) = a.first() {
        commands.entity(*entity).insert(HasFocus);
    }
}

pub fn vert_menu_controls(
    keys: Res<ButtonInput<KeyCode>>,
    elements: Query<(Entity, Option<&HasFocus>, &Transform), With<Focusable>>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::ArrowDown) {
        let mut a = elements
            .iter()
            .sort_by::<&Transform>(|a, b| a.translation.y.total_cmp(&b.translation.y))
            .collect::<Vec<_>>();
        if a.is_empty() {
            return;
        }
        let mut ready_to_focus = false;
        let mut entity_to_focus_next = None;
        if keys.just_pressed(KeyCode::ArrowUp) {
            a.reverse();
        }
        for (entity, focus, _) in &a {
            if ready_to_focus {
                entity_to_focus_next = Some(*entity);
                break;
            }
            if focus.is_some() {
                commands.entity(*entity).remove::<HasFocus>();
                ready_to_focus = true;
            }
        }
        let target_entity = entity_to_focus_next.unwrap_or(a[0].0);
        commands.entity(target_entity).insert(HasFocus);
    }
}
