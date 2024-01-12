use std::f32;

use bevy::window::PrimaryWindow;

use crate::*;

pub fn spawn_bullet(
    mut cmds: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    player_q: Query<&Transform, With<Player>>,
    mouse_button: Res<Input<MouseButton>>,
    char_texture: Res<AsciiSpriteSheet>,
    mut bullet_timer: ResMut<BulletSpawnTimer>,
    time: Res<Time>,
) {
    bullet_timer.tick(time.delta());

    if bullet_timer.finished() {
        let player_pos = player_q.single().translation.truncate();

        let window = window_q.single();
        let window_size = Vec2::new(
            window.physical_width() as f32,
            window.physical_height() as f32,
        );

        if let Some(mouse_pos) = window.cursor_position() {
            if mouse_button.pressed(MouseButton::Left) {
                // Reamap from window space to a vector around the player.
                let mut mouse_pos = mouse_pos - window_size / 2.;
                mouse_pos.y = -mouse_pos.y;
                let delta = mouse_pos.y.atan2(mouse_pos.x) - f32::consts::FRAC_PI_2;
                mouse_pos = mouse_pos.normalize_or_zero();

                let size = Vec2::new(7.5, 10.);
                let translation = player_pos + mouse_pos * (PLAYER_SIZE / 2. + 1.);
                let orient = Quat::from_rotation_z(delta);
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

                // Reset TIMER
                bullet_timer.reset();
            }
        }
    }
}
