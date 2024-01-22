use crate::prelude::*;
use bevy::window::PrimaryWindow;

pub fn setup_cursor(
    mut cmds: Commands,
    mut window_q: Query<&mut Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let mut window = window_q.single_mut();
    window.cursor.visible = false;
    let texture = asset_server.load("crosshairs.png");

    cmds.spawn(GameCursorBundle::new(texture));
}

pub fn move_cursor(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut cursor_q: Query<&mut Style, With<GameCursor>>,
) {
    let window = window_q.single();
    if let Some(real_cursor_pos) = window.cursor_position() {
        let mut cursor_style = cursor_q.single_mut();
        cursor_style.left = Val::Px(real_cursor_pos.x - 16.0);
        cursor_style.top = Val::Px(real_cursor_pos.y - 16.0);
    }
}
