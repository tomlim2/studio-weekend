use bevy::prelude::*;

use crate::gem::LevelUp;

const ATTACK_SPEED_PICK: f32 = 0.85; // cooldown multiplier per pick
const MOVE_SPEED_PICK: f32 = 1.15;
const PROJECTILE_COUNT_CAP: u32 = 5;

pub struct UpgradePlugin;

impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .insert_resource(Upgrades::default())
            .configure_sets(
                Update,
                GameplaySet.run_if(in_state(GameState::Playing)),
            )
            .add_systems(Update, listen_level_up.in_set(GameplaySet))
            .add_systems(OnEnter(GameState::LevelingUp), spawn_upgrade_ui)
            .add_systems(OnExit(GameState::LevelingUp), despawn_upgrade_ui)
            .add_systems(
                Update,
                pick_upgrade.run_if(in_state(GameState::LevelingUp)),
            )
            .add_systems(OnExit(GameState::GameOver), reset_upgrades);
    }
}

fn reset_upgrades(mut upgrades: ResMut<Upgrades>) {
    *upgrades = Upgrades::default();
}

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GameplaySet;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Playing,
    LevelingUp,
    GameOver,
}

#[derive(Resource)]
pub struct Upgrades {
    pub attack_cooldown_mult: f32,
    pub move_speed_mult: f32,
    pub projectile_count: u32,
}

impl Default for Upgrades {
    fn default() -> Self {
        Self {
            attack_cooldown_mult: 1.0,
            move_speed_mult: 1.0,
            projectile_count: 1,
        }
    }
}

#[derive(Component)]
struct UpgradeUi;

fn listen_level_up(
    mut events: MessageReader<LevelUp>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if events.read().last().is_some() {
        next_state.set(GameState::LevelingUp);
    }
}

fn spawn_upgrade_ui(mut commands: Commands) {
    commands
        .spawn((
            UpgradeUi,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.65)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("LEVEL UP — choose an upgrade"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            parent.spawn((
                Text::new("[1] Attack Speed +15%"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.95, 0.4)),
            ));
            parent.spawn((
                Text::new("[2] Move Speed +15%"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.4, 0.95, 0.4)),
            ));
            parent.spawn((
                Text::new("[3] Projectile Count +1"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.7, 1.0)),
            ));
        });
}

fn despawn_upgrade_ui(mut commands: Commands, q: Query<Entity, With<UpgradeUi>>) {
    for e in q.iter() {
        commands.entity(e).despawn();
    }
}

fn pick_upgrade(
    keys: Res<ButtonInput<KeyCode>>,
    mut upgrades: ResMut<Upgrades>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Digit1) {
        upgrades.attack_cooldown_mult *= ATTACK_SPEED_PICK;
        info!(
            "Picked: Attack Speed (cooldown mult={:.3})",
            upgrades.attack_cooldown_mult
        );
        next_state.set(GameState::Playing);
    } else if keys.just_pressed(KeyCode::Digit2) {
        upgrades.move_speed_mult *= MOVE_SPEED_PICK;
        info!(
            "Picked: Move Speed (mult={:.3})",
            upgrades.move_speed_mult
        );
        next_state.set(GameState::Playing);
    } else if keys.just_pressed(KeyCode::Digit3) {
        upgrades.projectile_count = (upgrades.projectile_count + 1).min(PROJECTILE_COUNT_CAP);
        info!("Picked: Projectile Count = {}", upgrades.projectile_count);
        next_state.set(GameState::Playing);
    }
}
