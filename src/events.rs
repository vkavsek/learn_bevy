use crate::prelude::*;

#[derive(Event, Deref, DerefMut)]
pub struct DespawnEventRecursive(pub Entity);

pub fn handle_despawn_event_recursive(
    mut cmds: Commands,
    mut despawn_events: EventReader<DespawnEventRecursive>,
) {
    for event in despawn_events.read() {
        if let Some(ent_cmd) = cmds.get_entity(**event) {
            ent_cmd.despawn_recursive();
        }
    }
}
