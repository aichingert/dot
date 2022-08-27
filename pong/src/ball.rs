use bevy::prelude::*;

use crate::states::GameState;
use crate::physics::*;

#[derive(Component)]
pub struct Ball;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_ball)
            .add_system_set(
                SystemSet::on_enter(GameState::Reset)
                    
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Reset)

            );
    }
}

fn spawn_ball(
    mut commands: Commands
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

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

fn remove_ball(
    mut commands: Commands,

) {

}