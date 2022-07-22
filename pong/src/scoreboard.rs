use bevy::prelude::*;

const FONT_SIZE: f32 = 40.0;

const TEXT_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

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
    commands
    .spawn()
    .insert_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Score: ".to_string(),
                    style: TextStyle {
                        font: default(),
                        font_size: 40.0,
                        color: TEXT_COLOR,
                    },
                },
                TextSection {
                    value: "".to_string(),
                    ..default()
                },
            ],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: SCOREBOARD_TEXT_PADDING,
                left: SCOREBOARD_TEXT_PADDING,
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
}


fn update_scoreboard(scoreboard: Res<Scoreboard>, mut text_query: Query<&mut Text>) {
    let mut text = text_query.single_mut();
    text.sections[1].value = format!("{}", scoreboard.score);
}