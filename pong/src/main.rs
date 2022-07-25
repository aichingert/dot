mod paddle;
mod scoreboard;
mod physics;
mod ball;
mod ui;

use bevy::{
    prelude::*,
    core::FixedTimestep,
};
use paddle::*;
use scoreboard::ScoreboardPlugin;
use physics::*;
use rand;
use ball::Ball;
use ui::UiPlugin;

const FPS: f32 = 1f32 / 60f32;

const BALL_SPEED: f32 = 500f32;

const LEFT_WALL: f32 = -450f32;
const RIGHT_WALL: f32 = 450f32;
const TOP_WALL: f32 = 300f32;
const BOTTOM_WALL: f32 = -300f32;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0f32, 0f32, 0f32)))
        .insert_resource(WindowDescriptor {
            title: "Pong".to_string(),
            width: 900f32,
            height: 600f32,
            resizable: false,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ScoreboardPlugin)
        .add_plugin(UiPlugin)
        .add_startup_system(setup)
        .add_startup_system(paddle_setup)
        .add_event::<CollisionEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(FPS as f64))
                .with_system(check_for_collisions)
                .with_system(paddle_movement.before(check_for_collisions))
                .with_system(apply_velocity.before(check_for_collisions))
        )
        .run();
}

fn setup(
    mut commands: Commands,
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
    .insert(Velocity(ball_velocity.normalize() * BALL_SPEED));

    //
    // Adding Walls
    //

    commands.spawn_bundle(WallBundle::new(WallLocation::Left));
    commands.spawn_bundle(WallBundle::new(WallLocation::Right));
    commands.spawn_bundle(WallBundle::new(WallLocation::Bottom));
    commands.spawn_bundle(WallBundle::new(WallLocation::Top));
}