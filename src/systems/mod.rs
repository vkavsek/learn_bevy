#![allow(clippy::type_complexity)]
pub mod debug;
pub mod enemy;
pub mod input;
pub mod map;
pub mod movement;
pub mod player;

use std::time::Duration;

use crate::prelude::*;

pub fn setup_game_cameras(mut cmds: Commands) {
    cmds.spawn(MainCamBundle::default());
    // cmds.spawn(MinimapCamBundle::default());
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

// TODO: get_component!
pub fn handle_player_enemy_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_q: Query<(&mut Health, Entity), (With<Player>, Without<Enemy>)>,
    mut enemy_q: Query<
        (
            &mut Health,
            &mut EnemyObjective,
            &mut ChangeStateTimer,
            &mut UnchangableTimer,
            &mut FollowTimer,
            Entity,
        ),
        With<Enemy>,
    >,
) {
    let (mut player_hp, player_ent) = player_q.single_mut();
    for event in collision_events.read() {
        if let CollisionEvent::Stopped(ent_a, ent_b, _) = event {
            // info!(
            //     "Collided: {:?}, {:?}, Player: {:?}",
            //     ent_a, ent_b, player_ent
            // );
            let (_found_player_ent, other_entity) = if ent_a == &player_ent {
                (ent_a, ent_b)
            } else if ent_b == &player_ent {
                (ent_b, ent_a)
            } else {
                return;
            };

            let find_enemy = enemy_q.get_mut(*other_entity);

            if let Ok((
                mut enemy_hp,
                mut objective,
                mut change_timer,
                mut unchangable_timer,
                mut follow_timer,
                _,
            )) = find_enemy
            {
                if let Some(unch_timer) = (*unchangable_timer).as_ref() {
                    if unch_timer.finished() {
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
                        unchangable_timer.take();
                    }
                } else {
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
                    *unchangable_timer = UnchangableTimer::new(Duration::from_millis(500));
                }
            }
        }
    }
}
