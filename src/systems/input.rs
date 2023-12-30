use crate::prelude::*;

/// TODO: Improve input handling
pub fn handle_input(
    keycode_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Velocity, With<Player>>,
    mut cam_query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    let mut vel = player_query.single_mut();

    let linvel = &mut vel.linvel;
    let mut add_to_vel = PLAYER_SPEED;
    if keycode_input.just_pressed(KeyCode::Space) {
        add_to_vel *= 10f32;
    }
    // RIGHT
    if keycode_input.any_pressed([KeyCode::D, KeyCode::Right]) {
        linvel.x += add_to_vel;
    }
    // LEFT
    if keycode_input.any_pressed([KeyCode::A, KeyCode::Left]) {
        linvel.x -= add_to_vel;
    }
    // UP
    if keycode_input.any_pressed([KeyCode::W, KeyCode::Up]) {
        linvel.y += add_to_vel;
    }
    // DOWN
    if keycode_input.any_pressed([KeyCode::S, KeyCode::Down]) {
        linvel.y -= add_to_vel;
    }

    if let Ok(mut projection) = cam_query.get_single_mut() {
        if keycode_input.pressed(KeyCode::Z) {
            projection.scale /= 1.05;
        }
        if keycode_input.pressed(KeyCode::U) {
            projection.scale *= 1.05;
        }
        projection.scale = projection.scale.clamp(0.25, 10.)
    }
}
