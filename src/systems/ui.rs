use crate::prelude::*;

pub fn setup_ui(mut cmds: Commands, sprites: Res<AsciiSpriteSheet>) {
    let mut z_index_offset = 50;
    let ui_root = cmds
        .spawn((
            UiRoot,
            NodeBundle {
                background_color: BackgroundColor(Color::RED),
                // border_color: todo!(),
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
        .id();

    cmds.entity(ui_root)
        .push_children(&[ui_wrapper_left, ui_wrapper_right]);
}
