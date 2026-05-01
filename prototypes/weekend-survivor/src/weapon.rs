use bevy::prelude::*;

use crate::enemy::Enemy;
use crate::player::Player;
use crate::upgrade::{GameplaySet, Upgrades};

pub const PROJECTILE_RADIUS: f32 = 5.0;
const PROJECTILE_SPEED: f32 = 600.0;
const PROJECTILE_LIFETIME_SECS: f32 = 1.5;
const PROJECTILE_COLOR: Color = Color::srgb(1.0, 0.95, 0.4);
const ATTACK_COOLDOWN_SECS: f32 = 0.6;
const ATTACK_RANGE: f32 = 500.0;
const PROJECTILE_SPREAD_RAD: f32 = 0.14; // ~8° between adjacent projectiles

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AttackTimer { since_last: 0.0 })
            .add_systems(
                Update,
                (auto_attack, move_projectiles, despawn_expired_projectiles)
                    .in_set(GameplaySet),
            );
    }
}

#[derive(Component)]
pub struct Projectile {
    pub velocity: Vec2,
    pub lifetime: Timer,
}

#[derive(Resource)]
struct AttackTimer {
    since_last: f32,
}

fn auto_attack(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    upgrades: Res<Upgrades>,
    mut timer: ResMut<AttackTimer>,
    player_q: Query<&Transform, With<Player>>,
    enemies: Query<&Transform, With<Enemy>>,
) {
    timer.since_last += time.delta_secs();
    let cooldown = ATTACK_COOLDOWN_SECS * upgrades.attack_cooldown_mult;
    if timer.since_last < cooldown {
        return;
    }
    let Ok(player_t) = player_q.single() else {
        return;
    };
    let player_pos = player_t.translation.truncate();
    let range_sq = ATTACK_RANGE * ATTACK_RANGE;

    let nearest = enemies
        .iter()
        .map(|t| {
            let pos = t.translation.truncate();
            (pos, pos.distance_squared(player_pos))
        })
        .filter(|(_, d2)| *d2 <= range_sq)
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(pos, _)| pos);

    let Some(target) = nearest else {
        return;
    };

    timer.since_last = 0.0;

    let base_dir = (target - player_pos).normalize_or_zero();
    if base_dir == Vec2::ZERO {
        return;
    }
    let count = upgrades.projectile_count.max(1);
    let center_offset = (count as f32 - 1.0) * 0.5;

    let mesh = meshes.add(Circle::new(PROJECTILE_RADIUS));
    let material = materials.add(PROJECTILE_COLOR);

    for i in 0..count {
        let angle_offset = (i as f32 - center_offset) * PROJECTILE_SPREAD_RAD;
        let rot = Mat2::from_angle(angle_offset);
        let dir = rot * base_dir;
        let velocity = dir * PROJECTILE_SPEED;

        commands.spawn((
            Projectile {
                velocity,
                lifetime: Timer::from_seconds(PROJECTILE_LIFETIME_SECS, TimerMode::Once),
            },
            Mesh2d(mesh.clone()),
            MeshMaterial2d(material.clone()),
            Transform::from_xyz(player_pos.x, player_pos.y, 0.0),
        ));
    }
}

fn move_projectiles(time: Res<Time>, mut q: Query<(&Projectile, &mut Transform)>) {
    let dt = time.delta_secs();
    for (proj, mut t) in &mut q {
        t.translation.x += proj.velocity.x * dt;
        t.translation.y += proj.velocity.y * dt;
    }
}

fn despawn_expired_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut q: Query<(Entity, &mut Projectile)>,
) {
    for (entity, mut proj) in &mut q {
        if proj.lifetime.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
