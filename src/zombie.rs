use bevy::prelude::*;
use rand::Rng;

pub const ZOMBIE_SPEED: f32 = 150.0;
pub const ZOMBIE_SPAWN_INTERVAL: f32 = 2.0;
pub const ZOMBIE_HEALTH: f32 = 100.0;

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

// Load zombie animation frames
pub fn setup_zombie_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let frames = (0..16)
        .map(|i| asset_server.load(&format!("tds_zombie/export/skeleton-move_{}.png", i)))
        .collect();
    commands.insert_resource(ZombieFrames(frames));
}

// Animate zombies
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

// Setup zombie spawn timer
pub fn setup_zombie_timer(mut commands: Commands) {
    commands.insert_resource(ZombieSpawnTimer(Timer::from_seconds(
        ZOMBIE_SPAWN_INTERVAL,
        TimerMode::Repeating,
    )));
}

// Spawn zombies with health bars
pub fn spawn_zombies(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<ZombieSpawnTimer>,
    frames: Res<ZombieFrames>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-375.0..375.0);

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
            health: ZOMBIE_HEALTH,
            max_health: ZOMBIE_HEALTH,
        })
        .id();

        // Spawn health bar as child
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

// Move zombies downwards
pub fn move_zombies(mut query: Query<&mut Transform, With<Zombie>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.translation.y -= ZOMBIE_SPEED * time.delta_seconds();
    }
}

// Update health bars to match current zombie health
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
