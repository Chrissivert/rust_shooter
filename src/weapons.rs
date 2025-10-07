use bevy::prelude::*;
use crate::score::Score;

#[derive(Component)]
pub struct WeaponButton {
    pub index: usize,
}

#[derive(Resource, Clone)]
pub struct WeaponAssets {
    pub lock_texture: Handle<Image>,
    pub weapon_textures: Vec<Handle<Image>>,
}


#[derive(Resource)]
pub struct Weapons {
    pub purchased: Vec<bool>, // unlocked/purchased weapons
    pub active: usize,        // index of currently selected weapon
    pub costs: Vec<u32>,      // cost for each weapon
}

pub fn setup_weapons(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let lock_texture = asset_server.load("images/lock.png");
    let weapon_images = vec![
        asset_server.load("images/pistol.png"),    // pistol
        asset_server.load("images/shotgun.png"),   // shotgun
        asset_server.load("images/minigun.png"),   // minigun
    ];

    let costs = vec![0, 100, 200]; // pistol is free, others have a cost

    // Weapons resource
    commands.insert_resource(Weapons {
        purchased: vec![true, false, false], // pistol unlocked by default
        active: 0,                            // pistol selected by default
        costs: costs.clone(),
    });

    let weapon_assets = WeaponAssets {
    lock_texture: lock_texture.clone(), // clone here
    weapon_textures: weapon_images.clone(),
};
commands.insert_resource(weapon_assets);


    // Spawn buttons
    for i in 0..weapon_images.len() {
        commands.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(50.0),
                    height: Val::Px(50.0),
                    margin: UiRect {
                        left: Val::Px(10.0),
                        top: Val::Px(10.0 + i as f32 * 60.0),
                        ..default()
                    },
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(3.0)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::GRAY.into(),
                ..default()
            },
            WeaponButton { index: i },
        ))
        .with_children(|parent| {
            // Weapon image
            parent.spawn(ImageBundle {
                image: UiImage::new(weapon_images[i].clone()),
                style: Style {
                    width: Val::Percent(80.0),
                    height: Val::Percent(80.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            });

            // Lock overlay
            parent.spawn(ImageBundle {
                image: UiImage::new(lock_texture.clone()),
                style: Style {
                    width: Val::Percent(80.0),
                    height: Val::Percent(80.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
                visibility: Visibility::Visible,
                ..default()
            });

            // Cost text
            parent.spawn(TextBundle::from_section(
                format!("{}$", costs[i]),
                TextStyle {
                    font_size: 10.0,
                    color: Color::WHITE,
                    ..default()
                },
            ))
            .insert(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(3.0),
                left: Val::Px(3.0),
                ..default()
            });
        });
    }
}

pub fn handle_weapon_input(
    keyboard: Res<Input<KeyCode>>,
    mut weapons: ResMut<Weapons>,
    mut score: ResMut<Score>,
) {
    for i in 0..weapons.purchased.len() {
        let key = match i {
            0 => KeyCode::Key1,
            1 => KeyCode::Key2,
            2 => KeyCode::Key3,
            _ => continue,
        };

        if keyboard.just_pressed(key) {
            if !weapons.purchased[i] {
                // Attempt purchase
                if score.0 >= weapons.costs[i] {
                    score.0 -= weapons.costs[i];
                    weapons.purchased[i] = true;
                    weapons.active = i;
                    println!("Purchased and selected weapon {}", i + 1);
                } else {
                    println!("Not enough score to purchase weapon {}", i + 1);
                }
            } else {
                weapons.active = i;
                println!("Selected weapon {}", i + 1);
            }
        }
    }
}

pub fn update_weapon_ui(
    weapons: Res<Weapons>,
    mut buttons: Query<(&WeaponButton, &mut BorderColor, &Children)>,
    mut images: Query<&mut Visibility, With<UiImage>>,
) {
    for (button, mut border, children) in buttons.iter_mut() {
        // Highlight border if active
        border.0 = if weapons.active == button.index {
            Color::YELLOW
        } else {
            Color::BLACK
        };

        // Update lock overlay visibility
        for (j, &child) in children.iter().enumerate() {
            if let Ok(mut visibility) = images.get_mut(child) {
                if j == 1 {
                    // Lock overlay is always child index 1
                    *visibility = if weapons.purchased[button.index] {
                        Visibility::Hidden
                    } else {
                        Visibility::Visible
                    };
                } else {
                    *visibility = Visibility::Visible;
                }
            }
        }
    }
}
