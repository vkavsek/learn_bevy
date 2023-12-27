use crate::prelude::*;

pub mod movement;
pub mod setup;

/// TODO: Improve input handling
pub fn handle_input(
    keycode_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Movement, With<Player>>,
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
}

// pub fn level_up(mut _query: Query<(&mut Xp, &mut Health)>) {
//     todo!()
// }
