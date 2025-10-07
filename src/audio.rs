use bevy::prelude::*;
use bevy::audio::{Audio, AudioSource, PlaybackSettings, PlaybackMode, Volume};
use rand::Rng;

#[derive(Resource)]
pub struct ZombieSounds {
    pub bullet: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct BackgroundMusic(pub Handle<AudioSource>);

pub fn setup_audio_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let bullet = asset_server.load("audio/bullet.mp3");
    let music = asset_server.load("audio/pvz-music.mp3");

    commands.insert_resource(ZombieSounds { bullet });
    commands.insert_resource(BackgroundMusic(music));
}

pub fn play_background_music(
    audio: Res<Audio>,
    music: Res<BackgroundMusic>,
) {
    audio.play_with_settings(
        music.0.clone(),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::new(1.0),
            speed: 1.0,
            paused: false,
            spatial: false,
        },
    );
}
