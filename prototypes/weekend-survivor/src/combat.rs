use std::collections::HashSet;

use bevy::prelude::*;

use crate::enemy::{ENEMY_RADIUS, Enemy, Health};
use crate::weapon::{PROJECTILE_RADIUS, Projectile};

const PROJECTILE_DAMAGE: i32 = 10;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, projectile_hits_enemy);
    }
}

fn projectile_hits_enemy(
    mut commands: Commands,
    projectiles: Query<(Entity, &Transform), With<Projectile>>,
    mut enemies: Query<(Entity, &Transform, &mut Health), With<Enemy>>,
) {
    let hit_radius = ENEMY_RADIUS + PROJECTILE_RADIUS;
    let hit_radius_sq = hit_radius * hit_radius;

    // Despawns are queued — entities stay queryable until the next sync point,
    // so we track in-frame kills to prevent two projectiles from "killing"
    // the same enemy this tick (which would log a stale-despawn warning).
    let mut killed_this_frame: HashSet<Entity> = HashSet::new();

    for (proj_entity, proj_t) in projectiles.iter() {
        let proj_pos = proj_t.translation.truncate();

        for (enemy_entity, enemy_t, mut health) in enemies.iter_mut() {
            if killed_this_frame.contains(&enemy_entity) {
                continue;
            }
            let enemy_pos = enemy_t.translation.truncate();
            if proj_pos.distance_squared(enemy_pos) > hit_radius_sq {
                continue;
            }

            health.0 -= PROJECTILE_DAMAGE;
            if health.0 <= 0 {
                commands.entity(enemy_entity).despawn();
                killed_this_frame.insert(enemy_entity);
            }
            commands.entity(proj_entity).despawn();
            break;
        }
    }
}
