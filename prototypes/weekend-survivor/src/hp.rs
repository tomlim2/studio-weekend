use bevy::prelude::*;

use crate::enemy::{ENEMY_RADIUS, Enemy};
use crate::player::{PLAYER_RADIUS, Player};
use crate::sfx::{Sfx, play_oneshot};
use crate::upgrade::{GameState, GameplaySet};

pub const PLAYER_MAX_HP: i32 = 100;
const ENEMY_TOUCH_DAMAGE: i32 = 10;
const I_FRAME_SECS: f32 = 0.5;

pub struct HpPlugin;

impl Plugin for HpPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SurvivalTime(0.0))
            .add_systems(
                Update,
                (
                    tick_survival_time,
                    tick_iframes,
                    enemy_touch_damage,
                    check_death,
                )
                    .chain()
                    .in_set(GameplaySet),
            )
            .add_systems(OnExit(GameState::GameOver), reset_survival);
    }
}

#[derive(Component)]
pub struct Hp {
    pub current: i32,
    pub max: i32,
}

impl Hp {
    pub fn new(max: i32) -> Self {
        Self { current: max, max }
    }
}

#[derive(Component, Default)]
pub struct IFrame(pub f32);

#[derive(Resource)]
pub struct SurvivalTime(pub f32);

fn tick_survival_time(time: Res<Time>, mut survival: ResMut<SurvivalTime>) {
    survival.0 += time.delta_secs();
}

fn tick_iframes(time: Res<Time>, mut q: Query<&mut IFrame>) {
    for mut f in &mut q {
        f.0 = (f.0 - time.delta_secs()).max(0.0);
    }
}

fn enemy_touch_damage(
    mut commands: Commands,
    sfx: Res<Sfx>,
    mut player_q: Query<(&Transform, &mut Hp, &mut IFrame), With<Player>>,
    enemies: Query<&Transform, With<Enemy>>,
) {
    let Ok((p_t, mut hp, mut iframe)) = player_q.single_mut() else {
        return;
    };
    if iframe.0 > 0.0 {
        return;
    }
    let p_pos = p_t.translation.truncate();
    let touch_sq = (PLAYER_RADIUS + ENEMY_RADIUS).powi(2);
    for e_t in enemies.iter() {
        let d_sq = e_t.translation.truncate().distance_squared(p_pos);
        if d_sq <= touch_sq {
            hp.current = (hp.current - ENEMY_TOUCH_DAMAGE).max(0);
            iframe.0 = I_FRAME_SECS;
            info!("Player hit! HP: {}/{}", hp.current, hp.max);
            play_oneshot(&mut commands, &sfx.hit);
            return;
        }
    }
}

fn check_death(
    player_q: Query<&Hp, With<Player>>,
    survival: Res<SurvivalTime>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok(hp) = player_q.single() else {
        return;
    };
    if hp.current <= 0 {
        info!("GAME OVER — survived {:.1}s", survival.0);
        next_state.set(GameState::GameOver);
    }
}

fn reset_survival(mut survival: ResMut<SurvivalTime>) {
    survival.0 = 0.0;
}
