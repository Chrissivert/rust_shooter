use bevy::prelude::*;
mod player; // import the player module
mod zombie; // your existing zombie.rs
mod gameover;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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
        // Systems
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, player::setup_player)
        .add_systems(Startup, zombie::setup_zombie_assets)
        .add_systems(Startup, zombie::setup_zombie_timer)
        .insert_resource(gameover::GameOver(false))


        .add_systems(Update, player::player_movement)
        .add_systems(Update, player::shooting)
        .add_systems(Update, player::move_bullets)
        .add_systems(Update, player::bullet_hit_zombie)
        .add_systems(Update, zombie::spawn_zombies)
        .add_systems(Update, zombie::animate_zombies)
        .add_systems(Update, zombie::move_zombies)
        .add_systems(Update, gameover::check_zombie_bottom)
        .add_systems(Update, gameover::show_game_over)
        .add_systems(Update, gameover::restart_game)
        .run();
}