use bevy::prelude::*;

use crate::GameState::Playing;

const FONT_SIZE: f32 = 40f32;
const COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct Score(pub f32, pub u8);
pub struct GameFinishedEvent;
pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(Playing)
                    .with_system(spawn_score)
            )
            .add_system_set(
                SystemSet::on_update(Playing)
                    .with_system(update_score)
            )
            .add_system_set(
                SystemSet::on_exit(Playing)
                    .with_system(clean_up_scoreboard)
            );
    }
}

impl Score {
    fn new(id: u8) -> Self {
        Self(0.0, id)
    }
}

impl Into<std::string::String> for Score {
    fn into(self) -> std::string::String {
        format!("{}", self.0)
    }
}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn game_finished_event(
    mut event_reader: &mut EventReader<GameFinishedEvent>,

) {
    
}

fn spawn_score(
    mut commands: Commands,
    materials: Res<AssetServer>
) {
    let text_style = TextStyle {
        font: materials.load("fonts/Lemon Days.otf"),
        font_size: FONT_SIZE,
        color: COLOR,
    };

    let box_size = Vec2::new(300.0, 200.0);
    let mut box_position = Vec2::new(-40.0, 280.0);

    let player_one_score: Score = Score::new(1);
    let player_two_score: Score = Score::new(2);
    
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(player_one_score, text_style.clone()),
        text_2d_bounds: bevy::text::Text2dBounds {
            size: box_size,
        },
        transform: Transform::from_xyz(
            box_position.x,
            box_position.y,
            1.0,
        ),
        ..default()
    })
    .insert(player_one_score);

    box_position = Vec2::new(40.0, 280.0);

    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(player_two_score, text_style.clone()),
        text_2d_bounds: bevy::text::Text2dBounds {
            size: box_size,
        },
        transform: Transform::from_xyz(
            box_position.x,
            box_position.y,
            1.0,
        ),
        ..default()
    })
    .insert(player_two_score);
}

fn update_score(
    mut query: Query<(&mut Text, &mut Score)>
){  
    query.for_each_mut( | (mut text, score) | {
        let value = score.0 as u32;
        let string = format!("{:01}", value);

        text.sections[0].value = string;
    });
}

fn clean_up_scoreboard(
    mut commands: Commands,
    score_query: Query<(Entity, &Score)>
) {
    // Iterate over all scores and remove them

    for (entity, _) in score_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}