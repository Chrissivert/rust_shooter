use bevy::prelude::*;
use rand::Rng; 

pub const ZOMBIE_SPEED: f32 = 150.0;
pub const ZOMBIE_SPAWN_INTERVAL: f32 = 2.0;

#[derive(Component)]
pub struct Zombie {
    pub current_frame: usize,
    pub timer: Timer,
}

#[derive(Resource)]
pub struct ZombieSpawnTimer(pub Timer);

#[derive(Resource)]
pub struct ZombieFrames(pub Vec<Handle<Image>>);

pub fn setup_zombie_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let frames = (0..16)
        .map(|i| asset_server.load(&format!("tds_zombie/export/skeleton-move_{}.png", i)))
        .collect();
    commands.insert_resource(ZombieFrames(frames));
}

pub fn animate_zombies(time: Res<Time>, frames: Res<ZombieFrames>, mut query: Query<(&mut Zombie, &mut Handle<Image>)>) {
    for (mut zombie, mut handle) in query.iter_mut() {
        if zombie.timer.tick(time.delta()).just_finished() {
            zombie.current_frame = (zombie.current_frame + 1) % frames.0.len();
            *handle = frames.0[zombie.current_frame].clone();
        }
    }
}

pub fn setup_zombie_timer(mut commands: Commands) {
    commands.insert_resource(ZombieSpawnTimer(Timer::from_seconds(ZOMBIE_SPAWN_INTERVAL, TimerMode::Repeating)));
}

pub fn spawn_zombies(mut commands: Commands, time: Res<Time>, mut timer: ResMut<ZombieSpawnTimer>, frames: Res<ZombieFrames>) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng(); 
        let x = rng.gen_range(-375.0..375.0);

        commands.spawn(SpriteBundle {
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
        });
    }
}

pub fn move_zombies(mut query: Query<&mut Transform, With<Zombie>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.translation.y -= ZOMBIE_SPEED * time.delta_seconds();
    }
}
