use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

use crate::prelude::*;

pub fn setup_debug_text(mut cmds: Commands) {
    // Create a <div> to hold the text
    let root = cmds
        .spawn((
            DebugRoot,
            NodeBundle {
                background_color: BackgroundColor(Color::BLUE.with_a(0.5)),
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
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    },
                    TextSection {
                        value: " N/A".into(),
                        style: TextStyle {
                            font_size: 16.0,
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

pub fn handle_debug_text(
    mut text_q: Query<&mut Text, With<DebugText>>,
    player_q: Query<&Transform, With<Player>>,
) {
    let player_pos = player_q.single().translation;
    for mut text in text_q.iter_mut() {
        text.sections[1].value = format!(
            "{:10.2}, {:10.2}, {:10.2}",
            player_pos.x, player_pos.y, player_pos.z
        );

        text.sections[1].style.color = Color::RED;
    }
}

pub fn fps_visibility(mut vis_q: Query<&mut Visibility, With<FpsRoot>>, kbd: Res<Input<KeyCode>>) {
    if kbd.just_pressed(KeyCode::F12) {
        let mut vis = vis_q.single_mut();
        *vis = match *vis {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
}
