use crate::prelude::*;

pub fn pause_game(
    keycode_input: Res<Input<KeyCode>>,
    current_state: Res<State<AppState>>,
    mut rapier_configuration: ResMut<RapierConfiguration>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    // ——————> PAUSE GAME
    if keycode_input.just_pressed(KeyCode::P) {
        match **current_state {
            AppState::Playing => {
                next_state.set(AppState::Paused);
                rapier_configuration.physics_pipeline_active = false;
            }
            AppState::Paused => {
                next_state.set(AppState::Playing);
                rapier_configuration.physics_pipeline_active = true;
            }
            _ => (),
        }
    }
}

pub fn handle_kbd_inputs(
    keycode_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Velocity, &mut GunType), With<Player>>,
    mut cam_query: Query<&mut OrthographicProjection, (With<MainCam>, Without<MinimapCam>)>,
) {
    let (mut vel, mut gun_type) = player_query.single_mut();

    // ——————> GUN TYPE
    if keycode_input.pressed(KeyCode::Key1) && !matches!(*gun_type, GunType::Pistol) {
        *gun_type = GunType::Pistol;
    }
    if keycode_input.pressed(KeyCode::Key2) && !matches!(*gun_type, GunType::Shotgun) {
        *gun_type = GunType::Shotgun;
    }
    if keycode_input.pressed(KeyCode::Key3) && !matches!(*gun_type, GunType::Ar) {
        *gun_type = GunType::Ar;
    }

    // ——————> MOVEMENT
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
