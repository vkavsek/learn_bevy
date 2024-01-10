use bevy::window::PrimaryWindow;

use crate::prelude::*;

/// TODO: Improve input handling
pub fn handle_kbd_inputs(
    keycode_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Velocity, With<Player>>,
    mut cam_query: Query<&mut OrthographicProjection, (With<MainCam>, Without<MinimapCam>)>,
) {
    let mut vel = player_query.single_mut();

    let add_to_vel = PLAYER_SPEED;

    let mut adding_vec = Vec2::ZERO;

    // RIGHT
    if keycode_input.any_pressed([KeyCode::D, KeyCode::Right]) {
        adding_vec.x += 1.0;
    }
    // LEFT
    if keycode_input.any_pressed([KeyCode::A, KeyCode::Left]) {
        adding_vec.x -= 1.0;
    }
    // UP
    if keycode_input.any_pressed([KeyCode::W, KeyCode::Up]) {
        adding_vec.y += 1.0;
    }
    // DOWN
    if keycode_input.any_pressed([KeyCode::S, KeyCode::Down]) {
        adding_vec.y -= 1.0;
    }

    vel.linvel += adding_vec.normalize_or_zero() * PLAYER_SPEED;

    // FIX: IF you mash SPACE everything goes to shit
    if keycode_input.just_pressed(KeyCode::Space) {
        vel.linvel *= 2.5;
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

pub fn handle_mouse_input(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mouse_button: Res<Input<MouseButton>>,
) {
    if let Some(pos) = window_q.single().cursor_position() {
        if mouse_button.just_pressed(MouseButton::Left) {
            println!("Clicked at {pos}");
        }
    }
}
