use bevy::{prelude::*, ui::update};

const FONT_SIZE: f32 = 20.0;
const TEXT_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
const SCORE_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(20.0);

pub struct Scoreboard {
    score: usize
}

pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Scoreboard { score: 10 })
            .add_startup_system(setup)
            .add_system(update_scoreboard);
    }
}

fn setup(
    mut commands:  Commands,

) {
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Score: ".to_string(),
                    style: TextStyle {
                        font_size: FONT_SIZE,
                        color: TEXT_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font_size: FONT_SIZE,
                        color: SCORE_COLOR,
                        ..default()
                    },
                },
            ],
            ..default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: SCOREBOARD_TEXT_PADDING,
                left: SCOREBOARD_TEXT_PADDING,
                ..default()
            },
            ..default()
        },
        ..default()
    });
}


fn update_scoreboard(scoreboard: Res<Scoreboard>, mut text_query: Query<&mut Text>) {
    let mut text = text_query.single_mut();
    text.sections[1].value = format!("{}", scoreboard.score);
}