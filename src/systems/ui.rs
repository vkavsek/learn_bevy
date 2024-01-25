use crate::prelude::*;

pub fn setup_ui(mut cmds: Commands, sprites: Res<AsciiSpriteSheet>) {
    let mut z_index_offset = 50;
    let ui_root = cmds
        .spawn((
            UiRoot,
            NodeBundle {
                z_index: ZIndex::Global(z_index_offset),
                style: Style {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    left: Val::Percent(0.),
                    right: Val::Percent(0.),
                    top: Val::Percent(80.),
                    bottom: Val::Percent(0.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .id();
    z_index_offset += 1;

    let player_info_text = cmds
        .spawn(TextBundle::from_sections([
            TextSection {
                value: "Player: ".into(),
                style: TextStyle {
                    font_size: 32.,
                    ..default()
                },
            },
            TextSection {
                value: "N/A".into(),
                style: TextStyle {
                    font_size: 32.,
                    ..default()
                },
            },
        ]))
        .id();
    let ui_wrapper_left = cmds
        .spawn((
            UiElement,
            NodeBundle {
                background_color: BackgroundColor(Color::BLUE),
                // border_color: todo!(),
                z_index: ZIndex::Global(z_index_offset),
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(50.),
                    height: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .add_child(player_info_text)
        .id();

    let score_text = cmds
        .spawn(TextBundle::from_sections([
            TextSection {
                value: "Score: ".into(),
                style: TextStyle {
                    font_size: 32.,
                    ..default()
                },
            },
            TextSection {
                value: "N/A".into(),
                style: TextStyle {
                    font_size: 32.,
                    ..default()
                },
            },
        ]))
        .id();
    let controls_text = cmds
        .spawn(TextBundle::from_sections([
            TextSection {
                value: "CONTROLS\n".into(),
                style: TextStyle {
                    font_size: 16.,
                    ..default()
                },
            },
            TextSection {
                value: "Movement: WASD\n".into(),
                style: TextStyle {
                    font_size: 16.,
                    ..default()
                },
            },
            TextSection {
                value: "Shoot: Left Click\n".into(),
                style: TextStyle {
                    font_size: 16.,
                    ..default()
                },
            },
            TextSection {
                value: "Change weapon: 1 / 2 / 3\n".into(),
                style: TextStyle {
                    font_size: 16.,
                    ..default()
                },
            },
            TextSection {
                value: "Toggle healthbars: H\n".into(),
                style: TextStyle {
                    font_size: 16.,
                    ..default()
                },
            },
            TextSection {
                value: "FPS + debug info: F12\n".into(),
                style: TextStyle {
                    font_size: 16.,
                    ..default()
                },
            },
            TextSection {
                value: "Physics debug: F10\n".into(),
                style: TextStyle {
                    font_size: 16.,
                    ..default()
                },
            },
            TextSection {
                value: "EGUI: F1\n".into(),
                style: TextStyle {
                    font_size: 16.,
                    ..default()
                },
            },
        ]))
        .id();
    let ui_wrapper_right = cmds
        .spawn((
            UiElement,
            NodeBundle {
                background_color: BackgroundColor(Color::GREEN),
                // border_color: todo!(),
                z_index: ZIndex::Global(z_index_offset),
                style: Style {
                    display: Display::Flex,
                    padding: UiRect::new(Val::Px(10.), Val::Px(10.), Val::Px(10.), Val::Px(10.)),
                    width: Val::Percent(50.),
                    height: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceAround,
                    ..default()
                },
                ..default()
            },
        ))
        .add_child(score_text)
        .add_child(controls_text)
        .id();

    cmds.entity(ui_root)
        .push_children(&[ui_wrapper_left, ui_wrapper_right]);
}
