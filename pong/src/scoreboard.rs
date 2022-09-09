use bevy::{
    prelude::*
};

const FONT_SIZE: f32 = 40f32;
const COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

#[derive(Component)]
pub struct Score(pub f32, pub u8);

pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(score_spawn)
            .add_system_set(
                SystemSet::new()
                    .with_system(update_score)
            );
    }
}

fn score_spawn(
    mut commands: Commands,
    materials: Res<AssetServer>
) {
    //Score value

    let text_style = TextStyle {
        font: materials.load("fonts/Lemon Days.otf"),
        font_size: FONT_SIZE,
        color: COLOR,
    };

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "",
                TextStyle {
                    font: materials.load("fonts/Lemon Days.otf"),
                    font_size: FONT_SIZE,
                    color: COLOR,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                }
            ),
            transform: Transform {
                translation: Vec3::new(-40.,280.,30.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Score(0., 1));

        commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "",
                TextStyle {
                    font: materials.load("fonts/Lemon Days.otf"),
                    font_size: FONT_SIZE,
                    color: COLOR,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                }
            ),
            transform: Transform {
                translation: Vec3::new(40.,280.,30.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Score(0., 2));
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