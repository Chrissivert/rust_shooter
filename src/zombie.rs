use bevy::prelude::*;
use rand::Rng;

// ---------------- Constants ----------------
pub const INITIAL_ZOMBIE_SPEED: f32 = 50.0;
pub const INITIAL_SPAWN_INTERVAL: f32 = 2.5;
pub const INITIAL_ZOMBIE_HEALTH: f32 = 50.0;
pub const RAMP_INTERVAL: f32 = 8.0;
pub const SPEED_INCREMENT: f32 = 10.0;
pub const SPAWN_DECREMENT: f32 = 0.2;
pub const HEALTH_INCREMENT: f32 = 20.0;

// ---------------- Components ----------------
#[derive(Component)]
pub struct Zombie { pub current_frame: usize, pub timer: Timer, pub health: f32, pub max_health: f32 }

#[derive(Component)]
pub struct HealthBar;

// ---------------- Resources ----------------
#[derive(Resource)] pub struct ZombieSpawnTimer(pub Timer);
#[derive(Resource)] pub struct ZombieFrames(pub Vec<Handle<Image>>);
#[derive(Resource)]
pub struct ZombieStats { pub speed: f32, pub spawn_interval: f32, pub health: f32, pub ramp_timer: Timer }

// ---------------- Startup ----------------
pub fn setup_zombie_stats(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ZombieStats {
        speed: INITIAL_ZOMBIE_SPEED,
        spawn_interval: INITIAL_SPAWN_INTERVAL,
        health: INITIAL_ZOMBIE_HEALTH,
        ramp_timer: Timer::from_seconds(RAMP_INTERVAL, TimerMode::Repeating),
    });

    let frames = (0..16).map(|i| asset_server.load(&format!("tds_zombie/export/skeleton-move_{}.png", i))).collect();
    commands.insert_resource(ZombieFrames(frames));
}

// ---------------- Difficulty ----------------
pub fn ramp_zombie_difficulty(time: Res<Time>, mut stats: ResMut<ZombieStats>, mut timer: ResMut<ZombieSpawnTimer>) {
    if stats.ramp_timer.tick(time.delta()).just_finished() {
        stats.speed += SPEED_INCREMENT;
        stats.spawn_interval = (stats.spawn_interval - SPAWN_DECREMENT).max(0.5);
        stats.health += HEALTH_INCREMENT;
        timer.0.set_duration(std::time::Duration::from_secs_f32(stats.spawn_interval));
    }
}

// ---------------- Spawning ----------------
pub fn spawn_zombies(mut commands: Commands, time: Res<Time>, mut timer: ResMut<ZombieSpawnTimer>, frames: Res<ZombieFrames>, stats: Res<ZombieStats>) {
    if timer.0.tick(time.delta()).just_finished() {
        let x = rand::thread_rng().random_range(-375.0..375.0);

        commands.spawn(SpriteBundle {
            texture: frames.0[0].clone(),
            transform: Transform {
                translation: Vec3::new(x, 250.0, 0.0),
                rotation: Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2),
                ..default()
            },
            sprite: Sprite { custom_size: Some(Vec2::splat(25.0)), ..default() },
            ..default()
        })
        .insert(Zombie { current_frame: 0, timer: Timer::from_seconds(0.1, TimerMode::Repeating), health: stats.health, max_health: stats.health })
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                transform: Transform::from_xyz(0.0, 20.0, 1.0),
                sprite: Sprite { color: Color::RED, custom_size: Some(Vec2::new(25.0, 4.0)), ..default() },
                ..default()
            }).insert(HealthBar);
        });
    }
}

// ---------------- Movement ----------------
pub fn move_zombies(mut query: Query<&mut Transform, With<Zombie>>, stats: Res<ZombieStats>, time: Res<Time>) {
    for mut t in query.iter_mut() { t.translation.y -= stats.speed * time.delta_seconds(); }
}

// ---------------- Health Bars ----------------
pub fn update_healthbars(zombies: Query<(&Zombie, &Children)>, mut bars: Query<&mut Sprite, With<HealthBar>>) {
    for (zombie, children) in zombies.iter() {
        for &child in children.iter() {
            if let Ok(mut sprite) = bars.get_mut(child) {
                sprite.custom_size = Some(Vec2::new(25.0 * (zombie.health / zombie.max_health), 4.0));
            }
        }
    }
}

// ---------------- Animation ----------------
pub fn animate_zombies(time: Res<Time>, frames: Res<ZombieFrames>, mut query: Query<(&mut Zombie, &mut Handle<Image>)>) {
    for (mut z, mut handle) in query.iter_mut() {
        if z.timer.tick(time.delta()).just_finished() {
            z.current_frame = (z.current_frame + 1) % frames.0.len();
            *handle = frames.0[z.current_frame].clone();
        }
    }
}
