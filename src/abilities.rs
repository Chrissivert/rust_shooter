use bevy::prelude::*;
use crate::score::Score;

#[derive(Component)]
pub struct AbilityButton {
    pub index: usize,
}

#[derive(Resource)]
pub struct AbilityAssets {
    pub lock_texture: Handle<Image>,
}

// pub fn load_ability_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let lock_texture = asset_server.load("images/lock.png"); // adjust path as needed
//     commands.insert_resource(AbilityAssets { lock_texture });
// }



#[derive(Resource)]
pub struct Abilities {
    pub purchased: Vec<bool>, // true if purchased
    pub active: Vec<bool>,    // true if currently enabled
    pub costs: Vec<u32>,      // cost for each ability
}

pub fn setup_abilities(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let lock_texture = asset_server.load("images/lock.png");

    // Ability images (make sure these exist in assets/images)
    let ability_images = vec![
        asset_server.load("images/shotgun.png"),
        asset_server.load("images/minigun.png"),
    ];

    let num_abilities = ability_images.len();
    let costs = vec![50, 100, 150, 200];

    commands.insert_resource(Abilities {
        purchased: vec![false; num_abilities],
        active: vec![false; num_abilities],
        costs: costs.clone(),
    });

    let ability_assets = AbilityAssets { lock_texture };

    for i in 0..num_abilities {
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
                    justify_content: JustifyContent::FlexEnd, // bottom text
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(3.0)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::GRAY.into(),
                ..default()
            },
            AbilityButton { index: i },
        ))
        .with_children(|parent| {
            // Ability image fills the button
            parent.spawn(ImageBundle {
                image: UiImage::new(ability_images[i].clone()),
                style: Style {
                    width: Val::Percent(80.0),
                    height: Val::Percent(80.0),
                    position_type: PositionType::Absolute, // fills entire button
                    ..default()
                },
                ..default()
            });

            // Lock overlay, also fills entire button
            parent.spawn(ImageBundle {
                image: UiImage::new(ability_assets.lock_texture.clone()),
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

            // Bottom text overlay (number + cost)
            parent.spawn(TextBundle::from_section(
                format!("{}: {}$", i + 1, costs[i]),
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






pub fn handle_abilities_input(
    keyboard: Res<Input<KeyCode>>,
    mut abilities: ResMut<Abilities>,
    mut score: ResMut<Score>,
) {
    for i in 0..abilities.purchased.len() {
        let key = match i {
            0 => KeyCode::Key1,
            1 => KeyCode::Key2,
            2 => KeyCode::Key3,
            3 => KeyCode::Key4,
            _ => continue,
        };

        if keyboard.just_pressed(key) {
            if !abilities.purchased[i] {
                // Attempt purchase
                if score.0 >= abilities.costs[i] {
                    score.0 -= abilities.costs[i];
                    abilities.purchased[i] = true;
                    abilities.active[i] = true;
                    println!("Purchased and activated ability {}", i + 1);
                } else {
                    println!("Not enough score to purchase ability {}", i + 1);
                }
            } else {
                // Toggle active state if already purchased
                abilities.active[i] = !abilities.active[i];
                println!(
                    "Ability {} is now {}",
                    i + 1,
                    if abilities.active[i] { "active" } else { "inactive" }
                );
            }
        }
    }
}

pub fn update_ability_ui(
    abilities: Res<Abilities>,
    mut buttons: Query<(&AbilityButton, &mut BorderColor, &Children)>,
    mut images: Query<&mut Visibility, With<UiImage>>,
) {
    for (button, mut border, children) in buttons.iter_mut() {
        border.0 = if abilities.active[button.index] {
            Color::YELLOW
        } else {
            Color::BLACK
        };

        for &child in children.iter() {
            if let Ok(mut visibility) = images.get_mut(child) {
                *visibility = if abilities.purchased[button.index] {
                    Visibility::Hidden
                } else {
                    Visibility::Visible
                };
            }
        }
    }
}


