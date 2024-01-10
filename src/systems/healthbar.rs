use crate::prelude::*;

pub fn handle_healthbars(
    mut healthbar_q: Query<(&Parent, &mut Sprite, &mut Transform), With<HealthBar>>,
    parent_q: Query<&Health, Changed<Health>>,
) {
    for (parent, mut sprite, mut transform) in healthbar_q.iter_mut() {
        if let Ok(parent_health) = parent_q.get(**parent) {
            let size = sprite
                .custom_size
                .expect("HealthBar should have a custom size");

            let piece = (parent_health.max - parent_health.current) as f32 * size.x
                / parent_health.max as f32;
            let width = size.x - piece;

            sprite.custom_size.as_mut().map(|size| size.x = width);
            transform.translation.x -= piece;
        }
    }
}

pub fn toggle_healthbar_vis(
    mut healthbar_q: Query<&mut Visibility, With<HealthBar>>,
    kbd_input: Res<Input<KeyCode>>,
) {
    if kbd_input.just_pressed(KeyCode::H) {
        for mut vis in healthbar_q.iter_mut() {
            *vis = match *vis {
                Visibility::Hidden => Visibility::Visible,
                _ => Visibility::Hidden,
            }
        }
    }
}
