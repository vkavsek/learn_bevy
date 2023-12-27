use drunk_game::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(BG_COLOR))
        .add_state::<AppState>()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: TITLE.into(),
                    resolution: WINDOW_RES.into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            GamePlugin,
        ))
        .run();
}
