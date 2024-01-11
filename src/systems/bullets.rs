use bevy::window::PrimaryWindow;

use crate::*;

pub fn spawn_bullet(
    mut cmds: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    player_q: Query<&Transform, With<Player>>,
    mouse_button: Res<Input<MouseButton>>,
    char_texture: Res<AsciiSpriteSheet>,
) {
    let player_pos = player_q.single().translation.truncate();

    let window = window_q.single();
    let window_size = Vec2::new(
        window.physical_width() as f32,
        window.physical_height() as f32,
    );

    if let Some(mouse_pos) = window.cursor_position() {
        if mouse_button.just_pressed(MouseButton::Left) {
            // Reamap from window space to a vector around the player.
            let mut mouse_pos = mouse_pos - window_size / 2.;
            mouse_pos.y = -mouse_pos.y;
            // FIX:
            // Compute the angle for sprite rotation
            let delta = mouse_pos.y.atan2(mouse_pos.x);
            mouse_pos = mouse_pos.normalize_or_zero();

            let size = Vec2::new(20., 20.);
            let translation = player_pos + mouse_pos * 30.;
            let orient = Quat::from_rotation_z(0.);
            let rotation = Quat::IDENTITY * orient;
            let target = mouse_pos;
            let bullet_speed = 1000.;

            cmds.spawn(BulletBundle::new(
                size,
                char_texture,
                translation,
                rotation,
                target,
                bullet_speed,
            ));
        }
    }
}
