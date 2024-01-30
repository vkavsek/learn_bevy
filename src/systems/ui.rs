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

pub fn handle_ui_guns(
    player_q: Query<&GunType, With<Player>>,
    mut ui_gun_root: Query<&Children, With<UiGunTypeRoot>>,
    mut ui_guns_set: ParamSet<(
        Query<&mut Style, With<UiGunPistol>>,
        Query<&mut Style, With<UiGunShotgun>>,
        Query<&mut Style, With<UiGunAr>>,
    )>,
    mut text_q: Query<(&mut Text, &Parent), With<UiGunText>>,
    bullet_timer: Res<BulletSpawnTimer>,
) {
    if let Ok(player_gun_type) = player_q.get_single() {
        for gun_root_children in ui_gun_root.iter_mut() {
            match player_gun_type {
                GunType::Pistol => {
                    let mut ui_pistol = ui_guns_set.p0();
                    for gr_child in gun_root_children.iter() {
                        if let Ok(mut style) = ui_pistol.get_mut(*gr_child) {
                            for (mut text, _) in text_q
                                .iter_mut()
                                .filter(|(_, text_parent)| ***text_parent == *gr_child)
                            {
                                text.sections[0].value = {
                                    let percent = bullet_timer.percent();
                                    if percent == 1. {
                                        "Pistol".into()
                                    } else {
                                        format!("{:.3}", percent)
                                    }
                                };
                            }
                            style.border = UiRect::all(Val::Px(1.));
                        }
                    }
                    for mut style in ui_guns_set.p1().iter_mut() {
                        style.border = UiRect::all(Val::Px(0.));
                    }
                    for mut style in ui_guns_set.p2().iter_mut() {
                        style.border = UiRect::all(Val::Px(0.));
                    }
                }
                GunType::Shotgun => {
                    let mut ui_shotgun = ui_guns_set.p1();
                    for gr_child in gun_root_children.iter() {
                        if let Ok(mut style) = ui_shotgun.get_mut(*gr_child) {
                            for (mut text, _) in text_q
                                .iter_mut()
                                .filter(|(_, text_parent)| ***text_parent == *gr_child)
                            {
                                text.sections[0].value = {
                                    let percent = bullet_timer.percent();
                                    if percent == 1. {
                                        "Shotgun".into()
                                    } else {
                                        format!("{:.3}", percent)
                                    }
                                };
                            }
                            style.border = UiRect::all(Val::Px(1.));
                        }
                    }
                    for mut style in ui_guns_set.p0().iter_mut() {
                        style.border = UiRect::all(Val::Px(0.));
                    }
                    for mut style in ui_guns_set.p2().iter_mut() {
                        style.border = UiRect::all(Val::Px(0.));
                    }
                }
                GunType::Ar => {
                    let mut ui_ar = ui_guns_set.p2();
                    for gr_child in gun_root_children.iter() {
                        if let Ok(mut style) = ui_ar.get_mut(*gr_child) {
                            for (mut text, _) in text_q
                                .iter_mut()
                                .filter(|(_, text_parent)| ***text_parent == *gr_child)
                            {
                                text.sections[0].value = {
                                    let percent = bullet_timer.percent();
                                    if percent == 1. {
                                        "AR".into()
                                    } else {
                                        format!("{:.3}", percent)
                                    }
                                };
                            }
                            style.border = UiRect::all(Val::Px(1.));
                        }
                    }
                    for mut style in ui_guns_set.p0().iter_mut() {
                        style.border = UiRect::all(Val::Px(0.));
                    }
                    for mut style in ui_guns_set.p1().iter_mut() {
                        style.border = UiRect::all(Val::Px(0.));
                    }
                }
            }
        }
    }
}

pub fn setup_ui_top(mut cmds: Commands) {
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
                    left: Val::Percent(0.),
                    top: Val::Percent(0.),
                    bottom: Val::Percent(90.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Start,
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
                align_items: AlignItems::Start,
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

    let ui_guns = cmds
        .spawn((
            UiGunTypeRoot,
            NodeBundle {
                z_index: ZIndex::Global(z_index_offset),
                style: Style { ..default() },
                ..default()
            },
        ))
        .with_children(|parent| {
            let guns = [GunType::Pistol, GunType::Shotgun, GunType::Ar];

            for gun in &guns {
                match gun {
                    GunType::Pistol => {
                        parent
                            .spawn((
                                NodeBundle {
                                    z_index: ZIndex::Global(z_index_offset),
                                    border_color: BorderColor(Color::YELLOW),
                                    style: Style {
                                        padding: UiRect::all(Val::Px(20.)),
                                        ..default()
                                    },
                                    ..default()
                                },
                                UiGunPistol,
                            ))
                            .with_children(|gun_wrapper| {
                                gun_wrapper.spawn((
                                    UiGunText,
                                    TextBundle::from_section(
                                        "Pistol",
                                        TextStyle {
                                            font_size: 16.,
                                            ..default()
                                        },
                                    ),
                                ));
                            });
                    }
                    GunType::Shotgun => {
                        parent
                            .spawn((
                                NodeBundle {
                                    z_index: ZIndex::Global(z_index_offset),
                                    border_color: BorderColor(Color::YELLOW),
                                    style: Style {
                                        padding: UiRect::all(Val::Px(20.)),
                                        ..default()
                                    },
                                    ..default()
                                },
                                UiGunShotgun,
                            ))
                            .with_children(|gun_wrapper| {
                                gun_wrapper.spawn((
                                    UiGunText,
                                    TextBundle::from_section(
                                        "Shotgun",
                                        TextStyle {
                                            font_size: 16.,
                                            ..default()
                                        },
                                    ),
                                ));
                            });
                    }
                    GunType::Ar => {
                        parent
                            .spawn((
                                NodeBundle {
                                    z_index: ZIndex::Global(z_index_offset),
                                    border_color: BorderColor(Color::YELLOW),
                                    style: Style {
                                        padding: UiRect::all(Val::Px(20.)),
                                        ..default()
                                    },
                                    ..default()
                                },
                                UiGunAr,
                            ))
                            .with_children(|gun_wrapper| {
                                gun_wrapper.spawn((
                                    UiGunText,
                                    TextBundle::from_section(
                                        "AR",
                                        TextStyle {
                                            font_size: 16.,
                                            ..default()
                                        },
                                    ),
                                ));
                            });
                    }
                }
            }
        })
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
        .add_child(ui_guns)
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
