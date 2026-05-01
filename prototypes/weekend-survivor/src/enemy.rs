use bevy::prelude::*;
use rand::Rng;

use crate::player::Player;
use crate::upgrade::GameplaySet;

const ENEMY_SIZE: f32 = 20.0;
pub const ENEMY_RADIUS: f32 = ENEMY_SIZE * 0.5;
const ENEMY_SPEED: f32 = 100.0;
const ENEMY_COLOR: Color = Color::srgb(0.9, 0.25, 0.25);
const ENEMY_HP: i32 = 10;
const SPAWN_INTERVAL_SECS: f32 = 0.5;
const SPAWN_RADIUS: f32 = 700.0;
const MAX_ENEMIES: usize = 100;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(
            SPAWN_INTERVAL_SECS,
            TimerMode::Repeating,
        )))
        .add_systems(Update, (spawn_enemies, chase_player).in_set(GameplaySet));
    }
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Health(pub i32);

#[derive(Resource)]
struct SpawnTimer(Timer);

fn spawn_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    enemies: Query<(), With<Enemy>>,
    player_q: Query<&Transform, With<Player>>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    if enemies.iter().count() >= MAX_ENEMIES {
        return;
    }
    let Ok(player_t) = player_q.single() else {
        return;
    };

    let mut rng = rand::thread_rng();
    let angle: f32 = rng.gen_range(0.0..std::f32::consts::TAU);
    let pos = player_t.translation.truncate()
        + Vec2::new(angle.cos(), angle.sin()) * SPAWN_RADIUS;

    commands.spawn((
        Enemy,
        Health(ENEMY_HP),
        Mesh2d(meshes.add(Rectangle::new(ENEMY_SIZE, ENEMY_SIZE))),
        MeshMaterial2d(materials.add(ENEMY_COLOR)),
        Transform::from_xyz(pos.x, pos.y, 0.0),
    ));
}

fn chase_player(
    time: Res<Time>,
    player_q: Query<&Transform, With<Player>>,
    mut enemies: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
) {
    let Ok(player_t) = player_q.single() else {
        return;
    };
    let target = player_t.translation.truncate();

    for mut t in &mut enemies {
        let pos = t.translation.truncate();
        let dir = (target - pos).normalize_or_zero();
        let delta = dir * ENEMY_SPEED * time.delta_secs();
        t.translation.x += delta.x;
        t.translation.y += delta.y;
    }
}
