use bevy::prelude::*;

//const FONT_SIZE: f32 = 40.0;

const TEXT_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

pub struct Scoreboard {
    score: usize
}

// region:    Components
#[derive(Component)]
struct Score(f32);
// endregion:    Components

pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Scoreboard { score: 10 })
            //.add_startup_system(setup)
            .add_startup_system(score_spawn)
            .add_system(update_score);
    }
}

fn score_spawn(
    mut commands: Commands,
    materials: Res<AssetServer>
) {
    //Score text
    commands.
        spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "Score: ",
                TextStyle {
                    font: materials.load("fonts/Perfect Island.otf"),
                    font_size: 50.0,
                    color: Color::rgb(0.0823, 0.0627, 0.1686),
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                }
            ),
            transform: Transform {
                translation: Vec3::new(-380.,190.,30.),
                ..Default::default()
            },
            ..Default::default()
        });
    //Score value
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "",
                TextStyle {
                    font: materials.load("fonts/Perfect Island.otf"),
                    font_size: 50.0,
                    color: Color::rgb(0.0823, 0.0627, 0.1686),
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                }
            ),
            transform: Transform {
                translation: Vec3::new(-255.,188.,30.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Score(0.));
}

fn update_score(
    time: Res<Time>,
    mut query: Query<(&mut Text, &mut Score)>
){  
    let (mut text, mut score) = query.single_mut();

    score.0 += time.delta_seconds();
    let value = score.0 as u32;
    let string = format!("{:05}", value);

    text.sections[0].value = string;
}