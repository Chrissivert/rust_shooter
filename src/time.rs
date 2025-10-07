use bevy::prelude::*;
use crate::gameover::GameOver;

#[derive(Resource)] pub struct SurvivalTime(pub f32);
#[derive(Component)] pub struct SurvivalTimerText;

pub fn setup_ui(mut commands: Commands) {
    commands.spawn(TextBundle {
        text: Text::from_section(
            "Time: 0.0",
            TextStyle { font: Default::default(), font_size: 30.0, color: Color::WHITE }
        ),
        style: Style { position_type: PositionType::Absolute, left: Val::Px(10.0), bottom: Val::Px(10.0), ..default() },
        ..default()
    }).insert(SurvivalTimerText);
}

pub fn update_survival_time(
    time: Res<Time>,
    mut survival_time: ResMut<SurvivalTime>,
    game_over: Res<GameOver>,
    mut query: Query<&mut Text, With<SurvivalTimerText>>
) {
    if !game_over.0 { survival_time.0 += time.delta_seconds(); }
    
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Time: {:.1} s", survival_time.0);
    }
}
