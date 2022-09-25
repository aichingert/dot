use bevy::prelude::*;

use crate::physics::*;
use crate::states::GameState;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ResetBallEvent>()
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(spawn_ball)
            );
    }
}

#[derive(Component)]
pub struct Ball;

#[derive(Default)]
pub struct ResetBallEvent;

pub fn reset_ball(
    mut event_reader: EventReader<ResetBallEvent>,
    mut ball_query: Query<(&Ball, &mut Transform)>
) {
    for _event in event_reader.iter() {
        for (_, mut transform) in ball_query.iter_mut() {
            transform.translation = Vec3::new(0.0, 0.0, 1.0);
        }
    }
}

fn spawn_ball(
    mut commands: Commands
) {
    commands.spawn_bundle(Camera2dBundle::default());

    let ball_velocity: Vec2 = Vec2::new(0.5, if rand::random() { -0.5 } else { 0.5 });

    commands
        .spawn()    
        .insert(Ball)
        .insert_bundle( SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.9, 0.9, 0.9),
                ..default()
            },
            transform: Transform { 
                translation: Vec3::new(0.0, 0.0, 1.0), 
                scale: Vec3::new(25.0, 25.0, 1.0),
                ..default()
            },
        ..default()
    })
    .insert(Velocity(ball_velocity.normalize() * super::BALL_SPEED));
}