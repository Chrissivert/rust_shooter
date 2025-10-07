use bevy::prelude::*;
use crate::zombie::Zombie;

pub const PLAYER_SPEED: f32 = 500.0;
pub const BULLET_SPEED: f32 = 800.0;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Bullet;

// Player movement system
pub fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = 0.0;
        if keyboard.pressed(KeyCode::Left) || keyboard.pressed(KeyCode::A) {
            direction -= 1.0;
        }
        if keyboard.pressed(KeyCode::Right) || keyboard.pressed(KeyCode::D) {
            direction += 1.0;
        }
        transform.translation.x += direction * PLAYER_SPEED * time.delta_seconds();
        transform.translation.x = transform.translation.x.clamp(-375.0, 375.0);
    }
}

pub fn shooting(
    keyboard: Res<Input<KeyCode>>,
    mut commands: Commands,
    query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for transform in query.iter() {
            // Spawn bullet
            commands.spawn(SpriteBundle {
                transform: Transform::from_xyz(transform.translation.x, transform.translation.y + 30., 0.),
                sprite: Sprite {
                    color: Color::rgb(1.0, 1.0, 0.0),
                    custom_size: Some(Vec2::new(3., 7.)),
                    ..default()
                },
                ..default()
            })
            .insert(Bullet);

            // Play bullet sound
            let bullet_sound = asset_server.load("audio/bullet.ogg");
            commands.spawn(AudioBundle {
                source: bullet_sound,
                settings: PlaybackSettings::default(),
            });
        }
    }
}


// Bullet movement system
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

// Bullet hits zombie system
pub fn bullet_hit_zombie(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    zombie_query: Query<(Entity, &Transform), With<Zombie>>,
) {
    for (b_entity, b_transform) in bullet_query.iter() {
        for (z_entity, z_transform) in zombie_query.iter() {
            let distance = b_transform.translation.distance(z_transform.translation);
            if distance < 25.0 {
                commands.entity(b_entity).despawn();
                commands.entity(z_entity).despawn();
            }
        }
    }
}



pub fn setup_player(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(0., -250., 0.),
        sprite: Sprite {
            color: Color::rgb(0.0, 0.0, 1.0),
            custom_size: Some(Vec2::new(10., 10.)), // Adjust size as needed
            ..default()
        },
        ..default()
    })
    .insert(Player);
}