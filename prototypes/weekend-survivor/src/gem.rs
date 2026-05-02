use bevy::prelude::*;

use crate::combat::EnemyKilled;
use crate::player::{Level, PLAYER_RADIUS, Player, Xp};
use crate::sfx::{Sfx, play_oneshot};
use crate::upgrade::{GameState, GameplaySet};

const GEM_RADIUS: f32 = 4.0;
const GEM_COLOR: Color = Color::srgb(0.4, 0.95, 0.4);
const MAGNET_RADIUS: f32 = 80.0;
const ATTRACT_SPEED: f32 = 400.0;
const PICKUP_RADIUS: f32 = PLAYER_RADIUS + GEM_RADIUS;
const XP_PER_GEM: u32 = 1;

pub struct GemPlugin;

impl Plugin for GemPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<LevelUp>()
            .add_systems(
                Update,
                (
                    spawn_gems_on_kill,
                    attract_gems_to_player,
                    pickup_gems,
                    level_up_check,
                )
                    .chain()
                    .in_set(GameplaySet),
            )
            .add_systems(OnExit(GameState::GameOver), reset_gems);
    }
}

#[derive(Message)]
pub struct LevelUp;

#[derive(Component)]
pub struct Gem;

fn spawn_gems_on_kill(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut events: MessageReader<EnemyKilled>,
) {
    for ev in events.read() {
        commands.spawn((
            Gem,
            Mesh2d(meshes.add(Circle::new(GEM_RADIUS))),
            MeshMaterial2d(materials.add(GEM_COLOR)),
            Transform::from_xyz(ev.pos.x, ev.pos.y, 0.0),
        ));
    }
}

fn attract_gems_to_player(
    time: Res<Time>,
    player_q: Query<&Transform, With<Player>>,
    mut gems: Query<&mut Transform, (With<Gem>, Without<Player>)>,
) {
    let Ok(player_t) = player_q.single() else {
        return;
    };
    let player_pos = player_t.translation.truncate();
    let radius_sq = MAGNET_RADIUS * MAGNET_RADIUS;
    let dt = time.delta_secs();

    for mut t in &mut gems {
        let pos = t.translation.truncate();
        if pos.distance_squared(player_pos) > radius_sq {
            continue;
        }
        let dir = (player_pos - pos).normalize_or_zero();
        let delta = dir * ATTRACT_SPEED * dt;
        t.translation.x += delta.x;
        t.translation.y += delta.y;
    }
}

fn pickup_gems(
    mut commands: Commands,
    sfx: Res<Sfx>,
    mut player_q: Query<(&Transform, &mut Xp), With<Player>>,
    gems: Query<(Entity, &Transform), With<Gem>>,
) {
    let Ok((player_t, mut xp)) = player_q.single_mut() else {
        return;
    };
    let player_pos = player_t.translation.truncate();
    let radius_sq = PICKUP_RADIUS * PICKUP_RADIUS;

    for (gem_entity, gem_t) in gems.iter() {
        if gem_t.translation.truncate().distance_squared(player_pos) <= radius_sq {
            xp.0 += XP_PER_GEM;
            commands.entity(gem_entity).despawn();
            info!("XP +{} (total {})", XP_PER_GEM, xp.0);
            play_oneshot(&mut commands, &sfx.pickup);
        }
    }
}

fn level_up_check(
    mut commands: Commands,
    sfx: Res<Sfx>,
    mut player_q: Query<(&mut Xp, &mut Level), With<Player>>,
    mut level_up_writer: MessageWriter<LevelUp>,
) {
    let Ok((mut xp, mut level)) = player_q.single_mut() else {
        return;
    };
    // Loop in case multiple level-ups in one tick (huge XP burst).
    loop {
        let threshold = 5 + level.0 * 3;
        if xp.0 < threshold {
            break;
        }
        xp.0 -= threshold;
        level.0 += 1;
        info!("LEVEL UP! Now level {}", level.0);
        level_up_writer.write(LevelUp);
        play_oneshot(&mut commands, &sfx.levelup);
    }
}

fn reset_gems(mut commands: Commands, gems: Query<Entity, With<Gem>>) {
    for e in gems.iter() {
        commands.entity(e).despawn();
    }
}
