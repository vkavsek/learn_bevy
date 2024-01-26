use crate::prelude::*;

pub fn handle_ui_player_hp(
    player_q: Query<&Health, (With<Player>, Changed<Health>)>,
    mut ui_q: Query<&mut Text, With<UiTextPlayerHp>>,
) {
    let Ok(player_health) = player_q.get_single() else {
        return;
    };
    for mut text in ui_q.iter_mut() {
        text.sections[1].value = format!(
            "{:10}",
            format!("{} / {}", player_health.current, player_health.max)
        );
    }
}

pub fn handle_ui_player_score(
    mut ui_q: Query<&mut Text, With<UiTextPlayerScore>>,
    score: Res<Score>,
) {
    for mut text in ui_q.iter_mut() {
        text.sections[1].value = format!("{:5}", **score);
    }
}

pub fn setup_ui_top(mut cmds: Commands, sprites: Res<AsciiSpriteSheet>) {
    let mut z_index_offset = 50;
    let ui_root = cmds
        .spawn((
            UiRootTop,
            NodeBundle {
                background_color: BackgroundColor(UI_BG_COLOR),
                z_index: ZIndex::Global(z_index_offset),
                style: Style {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    right: Val::Percent(70.),
                    left: Val::Percent(0.),
                    top: Val::Percent(0.),
                    bottom: Val::Percent(80.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .id();
    z_index_offset += 1;

    let player_name_text = cmds
        .spawn((
            UiTextPlayerName,
            TextBundle::from_sections([TextSection {
                value: PLAYER_NAME.into(),
                style: TextStyle {
                    font_size: 16.,
                    ..default()
                },
            }]),
        ))
        .id();
    let player_hp_text = cmds
        .spawn((
            UiTextPlayerHp,
            TextBundle::from_sections([
                TextSection {
                    value: "HP: ".into(),
                    style: TextStyle {
                        font_size: 16.,
                        ..default()
                    },
                },
                TextSection {
                    value: "N/A".into(),
                    style: TextStyle {
                        font_size: 16.,
                        ..default()
                    },
                },
            ]),
        ))
        .id();

    let ui_wrapper_left = cmds
        .spawn((NodeBundle {
            z_index: ZIndex::Global(z_index_offset),
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(40.)),
                width: Val::Percent(80.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                ..default()
            },
            ..default()
        },))
        .add_child(player_name_text)
        .add_child(player_hp_text)
        .id();

    cmds.entity(ui_root).push_children(&[ui_wrapper_left]);
}

pub fn setup_ui_bottom(mut cmds: Commands) {
    let mut z_index_offset = 50;
    let ui_root = cmds
        .spawn((
            UiRootBot,
            NodeBundle {
                z_index: ZIndex::Global(z_index_offset),
                background_color: BackgroundColor(UI_BG_COLOR),
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

    let score_text = cmds
        .spawn((
            UiTextPlayerScore,
            TextBundle::from_sections([
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
            ]),
        ))
        .id();
    let ui_wrapper_left = cmds
        .spawn((NodeBundle {
            // border_color: todo!(),
            z_index: ZIndex::Global(z_index_offset),
            style: Style {
                display: Display::Flex,
                padding: UiRect::all(Val::Px(20.)),
                // width: Val::Percent(80.),
                flex_grow: 1.0,
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                ..default()
            },
            ..default()
        },))
        // .add_child(player_name_text)
        .add_child(score_text)
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
        .spawn((NodeBundle {
            border_color: BorderColor(Color::WHITE),
            z_index: ZIndex::Global(z_index_offset),
            style: Style {
                border: UiRect::all(Val::Px(3.)),
                display: Display::Flex,
                padding: UiRect::all(Val::Px(20.)),
                width: Val::Auto,
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                ..default()
            },
            ..default()
        },))
        .add_child(controls_text)
        .id();

    cmds.entity(ui_root)
        .push_children(&[ui_wrapper_left, ui_wrapper_right]);
}
