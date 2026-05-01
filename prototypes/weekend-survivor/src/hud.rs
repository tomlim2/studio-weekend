use bevy::prelude::*;

use crate::hp::{Hp, PLAYER_MAX_HP, SurvivalTime};
use crate::player::Player;
use crate::upgrade::{GameState, GameplaySet};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hud)
            .add_systems(Update, update_hud.in_set(GameplaySet))
            .add_systems(OnEnter(GameState::GameOver), spawn_game_over_screen)
            .add_systems(OnExit(GameState::GameOver), despawn_game_over_screen)
            .add_systems(
                Update,
                listen_restart_input.run_if(in_state(GameState::GameOver)),
            );
    }
}

#[derive(Component)]
struct HpText;

#[derive(Component)]
struct TimerText;

#[derive(Component)]
struct GameOverScreen;

fn spawn_hud(mut commands: Commands) {
    commands.spawn((
        HpText,
        Text::new(format!("HP: {0}/{0}", PLAYER_MAX_HP)),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.4, 0.4)),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(16.0),
            top: Val::Px(16.0),
            ..default()
        },
    ));

    commands.spawn((
        TimerText,
        Text::new("00:00"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(16.0),
            left: Val::Percent(50.0),
            margin: UiRect::left(Val::Px(-30.0)),
            ..default()
        },
    ));
}

fn update_hud(
    player_q: Query<&Hp, With<Player>>,
    survival: Res<SurvivalTime>,
    mut hp_text: Query<&mut Text, (With<HpText>, Without<TimerText>)>,
    mut timer_text: Query<&mut Text, (With<TimerText>, Without<HpText>)>,
) {
    if let Ok(hp) = player_q.single() {
        if let Ok(mut t) = hp_text.single_mut() {
            **t = format!("HP: {}/{}", hp.current, hp.max);
        }
    }
    let secs = survival.0;
    let m = (secs / 60.0) as u32;
    let s = (secs % 60.0) as u32;
    if let Ok(mut t) = timer_text.single_mut() {
        **t = format!("{:02}:{:02}", m, s);
    }
}

fn spawn_game_over_screen(mut commands: Commands, survival: Res<SurvivalTime>) {
    let m = (survival.0 / 60.0) as u32;
    let s = (survival.0 % 60.0) as u32;
    commands
        .spawn((
            GameOverScreen,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("GAME OVER"),
                TextFont {
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::srgb(0.95, 0.3, 0.3)),
            ));
            p.spawn((
                Text::new(format!("Survived: {:02}:{:02}", m, s)),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            p.spawn((
                Text::new("[R] Restart"),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
        });
}

fn despawn_game_over_screen(
    mut commands: Commands,
    q: Query<Entity, With<GameOverScreen>>,
) {
    for e in q.iter() {
        commands.entity(e).despawn();
    }
}

fn listen_restart_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        next_state.set(GameState::Playing);
    }
}
