use bevy::prelude::*;

#[derive(Resource)]
pub struct Score(pub u32);

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct FloatingScore {
    pub timer: Timer,
}

pub fn update_floating_scores(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut FloatingScore, &mut Text)>,
) {
    for (entity, mut transform, mut fs, text) in query.iter_mut() {
        fs.timer.tick(time.delta());
        transform.translation.y += 20.0 * time.delta_seconds(); // move upward
        if fs.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}


pub fn setup_score_ui(mut commands: Commands) {
    commands.spawn(TextBundle {
        text: Text::from_section(
            "Score: 0",
            TextStyle {
                font: Default::default(),
                font_size: 30.0,
                color: Color::WHITE,
            }
        ),
        style: Style {
            position_type: PositionType::Absolute,
            right: Val::Px(10.0),
            bottom: Val::Px(10.0),
            ..default()
        },
        ..default()
    })
    .insert(ScoreText);
}

pub fn update_score_ui(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Score: {}", score.0);
    }
}

