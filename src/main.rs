use bevy::{
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    window::PresentMode,
};
use drunk_game::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(BG_COLOR))
        .add_state::<MapState>()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: TITLE.into(),
                        resolution: WINDOW_RES.into(),
                        present_mode: PresentMode::AutoVsync,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            // LogDiagnosticsPlugin::default(),
            // FrameTimeDiagnosticsPlugin,
            MapPlugin,
            GamePlugin,
            DebugPlugin,
        ))
        .run();
}
