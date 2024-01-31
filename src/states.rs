use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    // MainMenu,
    #[default]
    Build,
    Setup,
    Playing,
    Paused,
    GameOver,
}
