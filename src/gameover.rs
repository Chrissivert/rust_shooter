use bevy::prelude::*;
use crate::zombie::Zombie;
use crate::player::Bullet;
use crate::time::SurvivalTime;

#[derive(Resource)]
pub struct GameOver(pub bool);

#[derive(Component)]
pub struct GameOverUI;

#[derive(Component)]
pub struct RestartButton;

// Spawn Game Over UI
pub fn show_game_over(mut commands: Commands, game_over: Res<GameOver>, query: Query<Entity, With<GameOverUI>>) {
    if game_over.0 && query.is_empty() {
        commands.spawn((NodeBundle {
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
        }, GameOverUI))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section("Game Over!", TextStyle { font: Default::default(), font_size: 60.0, color: Color::WHITE }),
                ..default()
            });
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
            .insert(RestartButton)
            .with_children(|b| {
                b.spawn(TextBundle {
                    text: Text::from_section("Restart", TextStyle { font: Default::default(), font_size: 30.0, color: Color::BLACK }),
                    ..default()
                });
            });
        });
    }
}

// Restart game when button clicked
pub fn restart_game(mut commands: Commands,
                    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), With<RestartButton>>,
                    mut game_over: ResMut<GameOver>,
                    mut survival_time: ResMut<SurvivalTime>,
                    zombies: Query<Entity, With<Zombie>>,
                    bullets: Query<Entity, With<Bullet>>,
                    ui_elements: Query<Entity, With<GameOverUI>>) 
{
    for (interaction, _) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            game_over.0 = false;
            survival_time.0 = 0.0;
            for e in zombies.iter().chain(bullets.iter()).chain(ui_elements.iter()) { commands.entity(e).despawn_recursive(); }
        }
    }
}

// Check if any zombie reached bottom
pub fn check_zombie_bottom(mut game_over: ResMut<GameOver>, query: Query<&Transform, With<Zombie>>) {
    for t in query.iter() {
        if t.translation.y < -300.0 { game_over.0 = true; }
    }
}
