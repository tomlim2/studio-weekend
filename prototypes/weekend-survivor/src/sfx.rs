use bevy::prelude::*;

pub struct SfxPlugin;

impl Plugin for SfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_sfx);
    }
}

#[derive(Resource)]
pub struct Sfx {
    pub hit: Handle<AudioSource>,
    pub pickup: Handle<AudioSource>,
    pub levelup: Handle<AudioSource>,
}

fn load_sfx(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Sfx {
        hit: asset_server.load("sfx/hit.wav"),
        pickup: asset_server.load("sfx/pickup.wav"),
        levelup: asset_server.load("sfx/levelup.wav"),
    });
}

pub fn play_oneshot(commands: &mut Commands, handle: &Handle<AudioSource>) {
    commands.spawn((
        AudioPlayer::new(handle.clone()),
        PlaybackSettings::DESPAWN,
    ));
}
