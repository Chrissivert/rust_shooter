use bevy::prelude::*;
use rand::Rng;
use crate::{zombie::Zombie, score::{Score, FloatingScore}, abilities::Abilities};

pub const PLAYER_SPEED: f32 = 500.;
pub const BULLET_SPEED: f32 = 800.;

#[derive(Component)] pub struct Player;
#[derive(Component)] pub struct Bullet;
#[derive(Resource)] pub struct Weapon { pub is_minigun: bool, pub fire_timer: Timer }

pub fn setup_player(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(0., -250., 0.),
        sprite: Sprite { color: Color::BLUE, custom_size: Some(Vec2::new(10., 10.)), ..default() },
        ..default()
    }).insert(Player);
}

pub fn setup_weapon(mut commands: Commands) {
    commands.insert_resource(Weapon {
        is_minigun: false,
        fire_timer: Timer::from_seconds(0.05, TimerMode::Repeating),
    });
}

pub fn player_movement(keyboard: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Player>>, time: Res<Time>) {
    for mut t in &mut query {
        let dir = (keyboard.pressed(KeyCode::Right) || keyboard.pressed(KeyCode::D)) as i32 as f32
                - (keyboard.pressed(KeyCode::Left) || keyboard.pressed(KeyCode::A)) as i32 as f32;
        t.translation.x = (t.translation.x + dir * PLAYER_SPEED * time.delta_seconds()).clamp(-375., 375.);
    }
}

pub fn shooting(keyboard: Res<Input<KeyCode>>, mut commands: Commands, query: Query<&Transform, With<Player>>,
                asset_server: Res<AssetServer>, time: Res<Time>, abilities: Res<Abilities>, mut weapon: ResMut<Weapon>) {
    weapon.is_minigun = abilities.active.get(1).copied().unwrap_or(false);
    let shoot = if weapon.is_minigun {
        weapon.fire_timer.tick(time.delta()); keyboard.pressed(KeyCode::Space) && weapon.fire_timer.finished()
    } else { keyboard.just_pressed(KeyCode::Space) };

    if shoot {
        for transform in &query {
            commands.spawn(SpriteBundle {
                transform: Transform::from_xyz(transform.translation.x, transform.translation.y + 30., 0.),
                sprite: Sprite { color: Color::YELLOW, custom_size: Some(Vec2::new(3., 7.)), ..default() },
                ..default()
            }).insert(Bullet);
            let sound = asset_server.load("audio/bullet.ogg");
            commands.spawn(AudioBundle { source: sound, settings: PlaybackSettings::default(), ..default() });
        }
    }
}

pub fn move_bullets(mut commands: Commands, mut query: Query<(Entity, &mut Transform), With<Bullet>>, time: Res<Time>) {
    for (e, mut t) in &mut query {
        t.translation.y += BULLET_SPEED * time.delta_seconds();
        if t.translation.y > 300. { commands.entity(e).despawn(); }
    }
}

pub fn bullet_hit_zombie(mut commands: Commands, bullet_query: Query<(Entity, &Transform), With<Bullet>>,
                         mut zombie_query: Query<(Entity, &Transform, &mut Zombie)>, mut score: ResMut<Score>) {
    let mut rng = rand::thread_rng();
    for (b_e, b_t) in &bullet_query {
        for (z_e, z_t, mut z) in &mut zombie_query {
            if b_t.translation.distance(z_t.translation) < 25. {
                commands.entity(b_e).despawn();
                z.health -= 25.;
                score.0 += 10;
                spawn_floating_score(&mut commands, z_t.translation, "+10", Color::YELLOW, &mut rng);
                if z.health <= 0. {
                    commands.entity(z_e).despawn_recursive();
                    score.0 += 90;
                    spawn_floating_score(&mut commands, z_t.translation, "+100", Color::GOLD, &mut rng);
                }
            }
        }
    }
}

fn spawn_floating_score(commands: &mut Commands, pos: Vec3, text: &str, color: Color, rng: &mut impl Rng) {
    let offset = Vec3::new(rng.random_range(-10.0..10.0), rng.random_range(10.0..25.0), 1.);
    commands.spawn(Text2dBundle {
        text: Text::from_section(text, TextStyle { font: Default::default(), font_size: 20., color }),
        transform: Transform::from_translation(pos + offset),
        ..default()
    }).insert(FloatingScore { timer: Timer::from_seconds(0.5, TimerMode::Once) });
}
