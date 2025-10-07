use bevy::prelude::*;
use rand::Rng;
use crate::zombie::Zombie;
use crate::score::{Score, FloatingScore};
use crate::abilities::Abilities;

pub const PLAYER_SPEED: f32 = 500.0;
pub const BULLET_SPEED: f32 = 800.0;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Bullet;

#[derive(Resource)]
pub struct Weapon {
    pub is_minigun: bool, 
    pub fire_timer: Timer, 
}

pub fn setup_weapon(mut commands: Commands) {
    commands.insert_resource(Weapon {
        is_minigun: false,
        fire_timer: Timer::from_seconds(0.05, TimerMode::Repeating),
    });
}

// -------------------- Player Movement --------------------
pub fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = 0.0;
        if keyboard.pressed(KeyCode::Left) || keyboard.pressed(KeyCode::A) { direction -= 1.0; }
        if keyboard.pressed(KeyCode::Right) || keyboard.pressed(KeyCode::D) { direction += 1.0; }

        transform.translation.x = (transform.translation.x + direction * PLAYER_SPEED * time.delta_seconds())
            .clamp(-375.0, 375.0);
    }
}

// -------------------- Shooting --------------------
pub fn shooting(
    keyboard: Res<Input<KeyCode>>,
    mut commands: Commands,
    query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    abilities: Res<Abilities>,
    mut weapon: ResMut<Weapon>,
) {
    // Check if minigun is active
    weapon.is_minigun = abilities.active.get(1).copied().unwrap_or(false);

    if weapon.is_minigun {
        // Minigun: hold to shoot
        weapon.fire_timer.tick(time.delta());
        if keyboard.pressed(KeyCode::Space) && weapon.fire_timer.finished() {
            for transform in query.iter() {
                spawn_bullet(&mut commands, transform.translation);
                play_bullet_sound(&mut commands, &asset_server);
            }
        }
    } else {
        // Normal weapon: tap to shoot (ignore timer)
        if keyboard.just_pressed(KeyCode::Space) {
            for transform in query.iter() {
                spawn_bullet(&mut commands, transform.translation);
                play_bullet_sound(&mut commands, &asset_server);
            }
        }
    }
}



fn spawn_bullet(commands: &mut Commands, player_pos: Vec3) {
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(player_pos.x, player_pos.y + 30., 0.),
        sprite: Sprite {
            color: Color::YELLOW,
            custom_size: Some(Vec2::new(3., 7.)),
            ..default()
        },
        ..default()
    })
    .insert(Bullet);
}

fn play_bullet_sound(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let sound = asset_server.load("audio/bullet.ogg");
    commands.spawn(AudioBundle {
        source: sound,
        settings: PlaybackSettings::default(),
    });
}

// -------------------- Bullet Movement --------------------
pub fn move_bullets(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<Bullet>>,
    time: Res<Time>,
) {
    for (entity, mut transform) in query.iter_mut() {
        transform.translation.y += BULLET_SPEED * time.delta_seconds();
        if transform.translation.y > 300.0 {
            commands.entity(entity).despawn();
        }
    }
}

// -------------------- Bullet Hits Zombie --------------------
pub fn bullet_hit_zombie(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    mut zombie_query: Query<(Entity, &Transform, &mut Zombie)>,
    mut score: ResMut<Score>,
) {
    let mut rng = rand::rng();

    for (b_entity, b_transform) in bullet_query.iter() {
        for (z_entity, z_transform, mut zombie) in zombie_query.iter_mut() {
            if b_transform.translation.distance(z_transform.translation) < 25.0 {
                // Despawn bullet
                commands.entity(b_entity).despawn();

                // Apply damage and score for hit
                apply_damage(&mut zombie, 25.0);
                add_floating_score(&mut commands, z_transform.translation, "+10", Color::YELLOW, &mut rng);
                score.0 += 10;

                // If zombie is dead
                if zombie.health <= 0.0 {
                    commands.entity(z_entity).despawn_recursive();
                    add_floating_score(&mut commands, z_transform.translation, "+100", Color::GOLD, &mut rng);
                    score.0 += 90; // total 100
                }
            }
        }
    }
}

// -------------------- Helper Functions --------------------
fn apply_damage(zombie: &mut Zombie, amount: f32) {
    zombie.health -= amount;
}

fn add_floating_score(commands: &mut Commands, position: Vec3, text: &str, color: Color, rng: &mut impl Rng) {
    let x_offset = rng.random_range(-10.0..10.0);
    let y_offset = rng.random_range(10.0..25.0);

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            text,
            TextStyle {
                font: Default::default(),
                font_size: 20.0,
                color,
            },
        ),
        transform: Transform::from_translation(position + Vec3::new(x_offset, y_offset, 1.0)),
        ..default()
    })
    .insert(FloatingScore { timer: Timer::from_seconds(0.5, TimerMode::Once) });
}

// -------------------- Setup Player --------------------
pub fn setup_player(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(0., -250., 0.),
        sprite: Sprite {
            color: Color::BLUE,
            custom_size: Some(Vec2::new(10., 10.)),
            ..default()
        },
        ..default()
    })
    .insert(Player);
}
