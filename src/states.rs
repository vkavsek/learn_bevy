use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum SetupState {
    #[default]
    Build,
    Setup,
    Ready,
}
