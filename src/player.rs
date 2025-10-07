use bevy::prelude::*;
use rand::Rng;
use crate::{zombie::Zombie, score::{Score, FloatingScore}, weapons::Weapons};

pub const PLAYER_SPEED: f32 = 500.;
pub const BULLET_SPEED: f32 = 800.;

#[derive(Component)] pub struct Player;
#[derive(Component)] pub struct Bullet;
#[derive(Resource)] pub struct Weapon { pub is_minigun: bool, pub fire_timer: Timer }

#[derive(Component)]
pub struct WeaponSprite {
    pub pistol: Handle<Image>,
    pub minigun: Handle<Image>,
}


pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Load weapon images
    let pistol_handle = asset_server.load("images/pistol.png");
    let minigun_handle = asset_server.load("images/minigun.png");

    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(0., -250., 0.),
        sprite: Sprite { custom_size: Some(Vec2::new(12., 12.)), ..default() },
        ..default()
    })
    .insert(Player)
    .with_children(|parent| {
        parent.spawn(SpriteBundle {
    texture: pistol_handle.clone(),
    transform: Transform {
        translation: Vec3::new(0., 20., 1.),
        rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
        scale: Vec3::splat(5.0),
        ..default()
    },
    ..default()
})
.insert(WeaponSprite {
    pistol: pistol_handle,
    minigun: minigun_handle,
});
    });
}


pub fn update_weapon_sprite(
    weapon: Res<Weapon>,
    mut query: Query<(&mut Handle<Image>, &mut Sprite, &WeaponSprite)>
) {
    for (mut texture, mut sprite, weapon_sprite) in &mut query {
        if weapon.is_minigun {
            *texture = weapon_sprite.minigun.clone();  
            sprite.custom_size = Some(Vec2::new(8., 8.)); // minigun size
        } else {
            *texture = weapon_sprite.pistol.clone(); 
            sprite.custom_size = Some(Vec2::new(6., 6.)); // pistol size
        }
    }
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

pub fn shooting(
    keyboard: Res<Input<KeyCode>>,
    mut commands: Commands,
    query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    weapons: Res<Weapons>,
    mut weapon: ResMut<Weapon>,
) {
    // Update weapon type
    weapon.is_minigun = weapons.active == 2; // minigun index
    let is_shotgun = weapons.active == 1;    // shotgun index

    // Determine if shooting
    let shoot = if weapon.is_minigun {
        weapon.fire_timer.tick(time.delta());
        keyboard.pressed(KeyCode::Space) && weapon.fire_timer.finished()
    } else {
        keyboard.just_pressed(KeyCode::Space)
    };

    if shoot {
        for transform in &query {
            if is_shotgun {
                // Spawn multiple bullets with spread
                let angles: [f32; 5] = [-0.2, -0.1, 0.0, 0.1, 0.2]; // radians
                for &angle in &angles {
                    let t = transform.translation;
                    let dir = Vec3::new(angle.sin(), 1.0, 0.0).normalize();
                    commands.spawn(SpriteBundle {
                        transform: Transform::from_translation(t + Vec3::new(0., 30., 0.)),
                        sprite: Sprite { color: Color::ORANGE, custom_size: Some(Vec2::new(3., 7.)), ..default() },
                        ..default()
                    })
                    .insert(Bullet)
                    .insert(ShotgunBullet { direction: dir });
                }
            } else {
                // Regular bullet (pistol or minigun)
                commands.spawn(SpriteBundle {
                    transform: Transform::from_xyz(transform.translation.x, transform.translation.y + 30., 0.),
                    sprite: Sprite { color: Color::YELLOW, custom_size: Some(Vec2::new(3., 7.)), ..default() },
                    ..default()
                }).insert(Bullet);
            }

            // Play shooting sound
            let sound = asset_server.load("audio/bullet.ogg");
            commands.spawn(AudioBundle { source: sound, settings: PlaybackSettings::default(), ..default() });
        }
    }
}

#[derive(Component)]
pub struct ShotgunBullet {
    pub direction: Vec3,
}

// Update bullet movement for shotgun bullets
pub fn move_bullets(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, Option<&ShotgunBullet>), With<Bullet>>,
    time: Res<Time>
) {
    for (e, mut t, shotgun) in &mut query {
        let dir = if let Some(s) = shotgun {
            s.direction
        } else {
            Vec3::Y
        };
        t.translation += dir * BULLET_SPEED * time.delta_seconds();
        if t.translation.y > 300. || t.translation.x.abs() > 400. {
            commands.entity(e).despawn();
        }
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
