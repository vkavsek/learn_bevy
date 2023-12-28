use crate::prelude::*;

/// TODO: Improve input handling
pub fn handle_input(
    keycode_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Movement, With<Player>>,
    mut cam_query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    let mut movement = player_query.single_mut();

    movement.vel.x = 0.;
    movement.vel.y = 0.;
    // RIGHT
    if keycode_input.any_pressed([KeyCode::D, KeyCode::Right]) {
        movement.vel.x += 1.;
    }
    // LEFT
    if keycode_input.any_pressed([KeyCode::A, KeyCode::Left]) {
        movement.vel.x -= 1.;
    }
    // UP
    if keycode_input.any_pressed([KeyCode::W, KeyCode::Up]) {
        movement.vel.y += 1.;
    }
    // DOWN
    if keycode_input.any_pressed([KeyCode::S, KeyCode::Down]) {
        movement.vel.y -= 1.;
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
