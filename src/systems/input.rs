use crate::prelude::*;

/// TODO: Improve input handling
pub fn handle_input(
    keycode_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Velocity, &mut Speed), With<Player>>,
    mut cam_query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    let (mut vel, mut speed) = player_query.single_mut();

    vel.x = 0.;
    vel.y = 0.;
    // RIGHT
    if keycode_input.any_pressed([KeyCode::D, KeyCode::Right]) {
        vel.x += 1.;
    }
    // LEFT
    if keycode_input.any_pressed([KeyCode::A, KeyCode::Left]) {
        vel.x -= 1.;
    }
    // UP
    if keycode_input.any_pressed([KeyCode::W, KeyCode::Up]) {
        vel.y += 1.;
    }
    // DOWN
    if keycode_input.any_pressed([KeyCode::S, KeyCode::Down]) {
        vel.y -= 1.;
    }
    if keycode_input.just_pressed(KeyCode::Space) {
        **speed = PLAYER_SPEED * 15.;
    } else {
        **speed = PLAYER_SPEED;
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
