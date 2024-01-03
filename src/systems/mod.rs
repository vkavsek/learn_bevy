#![allow(clippy::type_complexity)]
pub mod enemy;
pub mod input;
pub mod map;
pub mod movement;
pub mod player;

use crate::prelude::*;

pub fn setup_game_cameras(mut cmds: Commands) {
    cmds.spawn(MainCamBundle::default());
    cmds.spawn(MinimapCamBundle::default());
}

pub fn load_spritesheet_texture(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
) {
    let image = asset_server.load::<Image>("tileset-16x16.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::splat(16.0), 16, 16, None, None);
    let atlas_handle = texture_atlas.add(atlas);
    commands.insert_resource(AsciiSpriteSheet(atlas_handle));
}

pub fn map_world_cleanup(mut commands: Commands, noise_map: Res<MapRootHandle>) {
    commands.entity(**noise_map).despawn_recursive();
}

pub fn handle_player_enemy_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_q: Query<(&mut Health, Entity), (With<Player>, Without<Enemy>)>,
    mut enemy_q: Query<
        (
            &mut Health,
            &mut EnemyObjective,
            &mut ChangeStateTimer,
            &mut FollowTimer,
            Entity,
        ),
        With<Enemy>,
    >,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Stopped(e_ent, p_ent, _) = event {
            let player = player_q.iter_mut().find(|(_, ent)| ent == p_ent);
            let enemy = enemy_q.iter_mut().find(|(_, _, _, _, ent)| ent == e_ent);
            if let Some((mut enemy_hp, mut objective, mut change_timer, mut follow_timer, _)) =
                enemy
            {
                if let Some((mut player_hp, _)) = player {
                    match *objective {
                        EnemyObjective::FollowPlayer => {
                            player_hp.current -= 1;
                            // Take from timer if enemy will get deactivated
                            change_timer.take();
                        }
                        EnemyObjective::Bounce => {
                            // Start timer since we will switch the objective below.
                            *change_timer = ChangeStateTimer::new(ENEMY_CHANGE_DELAY);
                        }
                    }
                    enemy_hp.current -= 1;
                    objective.switch();
                    // Reset follow timer if started.
                    follow_timer.take();
                }
            }
        }
    }
}
