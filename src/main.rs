use bevy::prelude::*;
use bevy::audio::{AudioBundle, PlaybackSettings, Volume};

mod player;
mod zombie;
mod gameover;
mod score;
mod time; 

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_background_music(asset_server: Res<AssetServer>, mut commands: Commands) {
    let music = asset_server.load("audio/pvz-music.ogg");

    commands.spawn(AudioBundle {
        source: music,
        settings: PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            volume: Volume::new_relative(0.5),
            ..default()
        },
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Zombie Shooter".to_string(),
                resolution: (800., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        // Startup systems
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_background_music)
        .add_systems(Startup, player::setup_player)
        .add_systems(Startup, zombie::setup_zombie_assets)
        .add_systems(Startup, zombie::setup_zombie_timer)
        .add_systems(Startup, time::setup_ui) 
        // Resources
        .insert_resource(gameover::GameOver(false))
        .insert_resource(time::SurvivalTime(0.0)) 
        .insert_resource(score::Score(0))
        // Player
        .add_systems(Update, player::player_movement)
        .add_systems(Update, player::shooting)
        .add_systems(Update, player::move_bullets)
        .add_systems(Update, player::bullet_hit_zombie)
        // Zombie
        .add_systems(Update, zombie::spawn_zombies)
        .add_systems(Update, zombie::animate_zombies)
        .add_systems(Update, zombie::move_zombies)
        .add_systems(Update, zombie::update_healthbars)
        // Timer
        .add_systems(Update, time::update_survival_time) 
        // Game
        .add_systems(Update, gameover::check_zombie_bottom)
        .add_systems(Update, gameover::show_game_over)
        .add_systems(Update, gameover::restart_game)

        .add_systems(Update, score:: update_floating_scores)
                .add_systems(Startup, score::setup_score_ui)
        .add_systems(Update, score::update_score_ui)

        .run();
}
