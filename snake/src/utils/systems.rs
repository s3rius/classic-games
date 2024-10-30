use bevy::prelude::*;

use super::components::HasFocus;

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn vert_menu_controls(
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Query<(Entity, Option<&HasFocus>, &Transform), With<Button>>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::ArrowDown) {
        let a = buttons
            .iter()
            .sort_by::<&Transform>(|a, b| a.translation.y.total_cmp(&b.translation.y))
            .collect::<Vec<_>>();
        if a.is_empty() {
            return;
        }
        let mut ready_to_focus = false;
        let mut entity_to_focus_next = None;
        for (entity, focus, _) in &a {
            if ready_to_focus {
                entity_to_focus_next = Some(*entity);
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
