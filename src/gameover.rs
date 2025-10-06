use bevy::prelude::*;
use crate::zombie::Zombie;
use crate::player::{Bullet};

// Resource tracking game state
#[derive(Resource)]
pub struct GameOver(pub bool);

// Marker component for the Game Over UI
#[derive(Component)]
pub struct GameOverUI;

// System: show Game Over UI when game is over
pub fn show_game_over(
    mut commands: Commands,
    game_over: Res<GameOver>,
    query: Query<Entity, With<GameOverUI>>,
    asset_server: Res<AssetServer>,
) {
    if game_over.0 && query.is_empty() {
        // Root node
        commands.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
                ..default()
            },
            GameOverUI,
        ))
        .with_children(|parent| {
            // Game Over text
            parent.spawn(TextBundle {
    text: Text::from_section(
        "Game Over!",
        TextStyle {
            font: Default::default(), // default font
            font_size: 60.0,
            color: Color::WHITE,
        },
    ),
    ..default()
});


            // Restart button
            parent.spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(50.0),
                    margin: UiRect::all(Val::Px(20.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::GRAY.into(),
                ..default()
            })
            .with_children(|b| {
                b.spawn(TextBundle {
                    text: Text::from_section(
                        "Restart",
                        TextStyle {
                            font: Default::default(),
                            font_size: 30.0,
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                });
            });
        });
    }
}

// System: restart the game when button is clicked
pub fn restart_game(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>)
    >,
    mut game_over: ResMut<GameOver>,
    zombies: Query<Entity, With<Zombie>>,
    bullets: Query<Entity, With<Bullet>>,
    ui_elements: Query<Entity, With<GameOverUI>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            // Reset game state
            game_over.0 = false;

            // Despawn all zombies
            for entity in zombies.iter() {
                commands.entity(entity).despawn();
            }

            // Despawn all bullets
            for entity in bullets.iter() {
                commands.entity(entity).despawn();
            }

            // Despawn Game Over UI
            for entity in ui_elements.iter() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

// System: check if any zombie reached bottom
pub fn check_zombie_bottom(
    mut game_over: ResMut<GameOver>,
    query: Query<&Transform, With<Zombie>>,
) {
    for transform in query.iter() {
        if transform.translation.y < -300.0 {
            game_over.0 = true;
        }
    }
}
