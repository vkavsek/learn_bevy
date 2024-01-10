use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

use crate::prelude::*;

pub fn setup_debug_text(mut cmds: Commands) {
    // Create a <div> to hold the text
    let root = cmds
        .spawn((
            DebugRoot,
            NodeBundle {
                background_color: BackgroundColor(Color::BLUE.with_a(0.9)),
                // Max Z-index to render on top of everything
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Percent(1.),
                    bottom: Val::Percent(1.),
                    top: Val::Auto,
                    left: Val::Auto,
                    padding: UiRect::all(Val::Px(4.0)),
                    border: UiRect::all(Val::Px(1.)),
                    ..Default::default()
                },
                border_color: BorderColor(Color::WHITE.with_a(0.5)),
                ..Default::default()
            },
        ))
        .id();
    let debug_text = cmds
        .spawn((
            DebugText,
            TextBundle {
                text: Text::from_sections([
                    TextSection {
                        value: "Player position: ".into(),
                        style: TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    },
                    TextSection {
                        value: " N/A".into(),
                        style: TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    },
                    TextSection {
                        value: "\nTilePos: ".into(),
                        style: TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    },
                    TextSection {
                        value: " N/A".into(),
                        style: TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    },
                    TextSection {
                        value: "\nNoiseVal: ".into(),
                        style: TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    },
                    TextSection {
                        value: " N/A".into(),
                        style: TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    },
                    TextSection {
                        value: "\nDamping: ".into(),
                        style: TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    },
                    TextSection {
                        value: " N/A".into(),
                        style: TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();

    cmds.entity(root).add_child(debug_text);
}

pub fn setup_fps_counter(mut cmds: Commands) {
    // Create a <div> to hold the text
    let root = cmds
        .spawn((
            FpsRoot,
            NodeBundle {
                background_color: BackgroundColor(Color::BLUE.with_a(0.75)),
                // Max Z-index to render on top of everything
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Percent(1.),
                    top: Val::Percent(1.),
                    left: Val::Auto,
                    bottom: Val::Auto,
                    padding: UiRect::all(Val::Px(4.0)),
                    border: UiRect::all(Val::Px(3.)),
                    ..Default::default()
                },
                border_color: BorderColor(Color::BLACK),
                ..Default::default()
            },
        ))
        .id();
    let text_fps = cmds
        .spawn((
            FpsText,
            TextBundle {
                text: Text::from_sections([
                    TextSection {
                        value: "FPS: ".into(),
                        style: TextStyle {
                            font_size: 24.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    },
                    TextSection {
                        value: " N/A".into(),
                        style: TextStyle {
                            font_size: 24.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();

    cmds.entity(root).add_child(text_fps);
}

pub fn handle_fps_update(
    diagnostics: Res<DiagnosticsStore>,
    mut text_q: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut text_q {
        // try to get a "smoothed" FPS value from Bevy
        if let Some(value) = diagnostics
            .get(FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            // Format the number as to leave space for 4 digits, just in case,
            // right-aligned and rounded. This helps readability when the
            // number changes rapidly.
            text.sections[1].value = format!("{value:>4.0}");

            // Let's make it extra fancy by changing the color of the
            // text according to the FPS value:
            text.sections[1].style.color = if value >= 120.0 {
                // Above 120 FPS, use green color
                Color::rgb(0.0, 1.0, 0.0)
            } else if value >= 60.0 {
                // Between 60-120 FPS, gradually transition from yellow to green
                Color::rgb((1.0 - (value - 60.0) / (120.0 - 60.0)) as f32, 1.0, 0.0)
            } else if value >= 30.0 {
                // Between 30-60 FPS, gradually transition from red to yellow
                Color::rgb(1.0, ((value - 30.0) / (60.0 - 30.0)) as f32, 0.0)
            } else {
                // Below 30 FPS, use red color
                Color::rgb(1.0, 0.0, 0.0)
            }
        } else {
            // display "N/A" if we can't get a FPS measurement
            // add an extra space to preserve alignment
            text.sections[1].value = " N/A".into();
            text.sections[1].style.color = Color::WHITE;
        }
    }
}

pub fn update_debug_text(
    mut text_q: Query<&mut Text, With<DebugText>>,
    player_q: Query<(&Transform, &Damping, &PlayerNoiseDebug), With<Player>>,
) {
    let player = player_q.single();
    let player_pos = player.0.translation;
    let damping = player.1;
    let noise_debug = player.2;

    let (x, y) = (
        (((player_pos.x + MAP_SIZE_PX.x / 2.0) / GRID_SIZE.x).floor() as u32),
        (((player_pos.y + MAP_SIZE_PX.y / 2.0) / GRID_SIZE.y).floor() as u32),
    );
    for mut text in text_q.iter_mut() {
        text.sections[1].value = format!(
            "{:10.2}, {:10.2}, {:10.2}",
            player_pos.x, player_pos.y, player_pos.z
        );
        text.sections[1].style.color = Color::ORANGE;

        text.sections[3].value = format!("{x:20}, {y:20}");
        text.sections[3].style.color = Color::ORANGE;

        text.sections[5].value = format!("{:30.4}", noise_debug.unwrap_or(999.).abs());
        text.sections[5].style.color = match noise_debug.unwrap_or(999.).abs() {
            v if v < 0.1 => Color::hex("#00ff00"),
            v if v < 0.2 => Color::hex("#3ff03f"),
            v if v < 0.25 => Color::hex("b35900"),
            v if v < 0.3 => Color::hex("#ffff1a"),
            v if v <= 1.0 => Color::hex("#8080ff"),
            _ => Color::hex("#FF0000"),
        }
        .unwrap();

        text.sections[7].value = format!("{:30.1}", damping.linear_damping);
    }
}

pub fn debug_info_visibility(
    mut fps_vis_q: Query<&mut Visibility, (With<FpsRoot>, Without<DebugRoot>)>,
    mut debug_vis_q: Query<&mut Visibility, (With<DebugRoot>, Without<FpsRoot>)>,
    kbd: Res<Input<KeyCode>>,
) {
    if kbd.just_pressed(KeyCode::F12) {
        let mut vis = fps_vis_q.single_mut();
        let mut d_vis = debug_vis_q.single_mut();
        *vis = match *vis {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };
        *d_vis = match *d_vis {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
}
