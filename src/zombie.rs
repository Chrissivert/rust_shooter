use bevy::prelude::*;
use rand::Rng;

// ---------------- Constants ----------------
pub const INITIAL_ZOMBIE_SPEED: f32 = 50.0;
pub const INITIAL_SPAWN_INTERVAL: f32 = 2.5;
pub const INITIAL_ZOMBIE_HEALTH: f32 = 50.0;
pub const RAMP_INTERVAL: f32 = 8.0; // seconds
pub const SPEED_INCREMENT: f32 = 10.0;
pub const SPAWN_DECREMENT: f32 = 0.2; // spawn interval decreases
pub const HEALTH_INCREMENT: f32 = 20.0;

// ---------------- Components ----------------
#[derive(Component)]
pub struct Zombie {
    pub current_frame: usize,
    pub timer: Timer,
    pub health: f32,
    pub max_health: f32,
}

#[derive(Component)]
pub struct HealthBar;

#[derive(Resource)]
pub struct ZombieSpawnTimer(pub Timer);

#[derive(Resource)]
pub struct ZombieFrames(pub Vec<Handle<Image>>);

// Resource to store dynamic zombie stats
#[derive(Resource)]
pub struct ZombieStats {
    pub speed: f32,
    pub spawn_interval: f32,
    pub health: f32,
    pub ramp_timer: Timer,
}

// ---------------- Startup Systems ----------------
pub fn setup_zombie_stats(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Insert dynamic zombie stats
    commands.insert_resource(ZombieStats {
        speed: INITIAL_ZOMBIE_SPEED,
        spawn_interval: INITIAL_SPAWN_INTERVAL,
        health: INITIAL_ZOMBIE_HEALTH,
        ramp_timer: Timer::from_seconds(RAMP_INTERVAL, TimerMode::Repeating),
    });

    // Load zombie animation frames
    let frames: Vec<Handle<Image>> = (0..16)
        .map(|i| asset_server.load(&format!("tds_zombie/export/skeleton-move_{}.png", i)))
        .collect();

    // Insert frames as a resource
    commands.insert_resource(ZombieFrames(frames));
}


// ---------------- Difficulty Ramp System ----------------
pub fn ramp_zombie_difficulty(time: Res<Time>, mut stats: ResMut<ZombieStats>, mut spawn_timer: ResMut<ZombieSpawnTimer>) {
    if stats.ramp_timer.tick(time.delta()).just_finished() {
        stats.speed += SPEED_INCREMENT;
        stats.spawn_interval = (stats.spawn_interval - SPAWN_DECREMENT).max(0.5);
        stats.health += HEALTH_INCREMENT;

        spawn_timer.0.set_duration(std::time::Duration::from_secs_f32(stats.spawn_interval));
    }
}

// ---------------- Spawn Zombies ----------------
pub fn spawn_zombies(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<ZombieSpawnTimer>,
    frames: Res<ZombieFrames>,
    stats: Res<ZombieStats>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::rng();
        let x = rng.random_range(-375.0..375.0);

        let zombie_entity = commands.spawn(SpriteBundle {
            texture: frames.0[0].clone(),
            transform: Transform {
                translation: Vec3::new(x, 250.0, 0.0),
                rotation: Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2),
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(25.0, 25.0)),
                ..default()
            },
            ..default()
        })
        .insert(Zombie {
            current_frame: 0,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            health: stats.health,
            max_health: stats.health,
        })
        .id();

        // Health bar as child
        commands.entity(zombie_entity).with_children(|parent| {
            parent.spawn(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 20.0, 1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(25.0, 4.0)),
                    ..default()
                },
                ..default()
            })
            .insert(HealthBar);
        });
    }
}

// ---------------- Move Zombies ----------------
pub fn move_zombies(mut query: Query<&mut Transform, With<Zombie>>, stats: Res<ZombieStats>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.translation.y -= stats.speed * time.delta_seconds();
    }
}


pub fn update_healthbars(
    zombie_query: Query<(&Zombie, &Children)>,
    mut healthbar_query: Query<&mut Sprite, With<HealthBar>>,
) {
    for (zombie, children) in zombie_query.iter() {
        for &child in children.iter() {
            if let Ok(mut sprite) = healthbar_query.get_mut(child) {
                let health_ratio = zombie.health / zombie.max_health;
                sprite.custom_size = Some(Vec2::new(25.0 * health_ratio, 4.0));
            }
        }
    }
}


pub fn animate_zombies(
    time: Res<Time>,
    frames: Res<ZombieFrames>,
    mut query: Query<(&mut Zombie, &mut Handle<Image>)>
) {
    for (mut zombie, mut handle) in query.iter_mut() {
        if zombie.timer.tick(time.delta()).just_finished() {
            zombie.current_frame = (zombie.current_frame + 1) % frames.0.len();
            *handle = frames.0[zombie.current_frame].clone();
        }
    }
}


