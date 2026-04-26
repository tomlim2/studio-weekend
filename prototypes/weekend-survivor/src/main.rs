use bevy::prelude::*;

mod camera;
mod enemy;
mod player;

use camera::CameraPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Weekend Survivor".into(),
                resolution: (1280u32, 720u32).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((CameraPlugin, PlayerPlugin, EnemyPlugin))
        .run();
}
