mod paddle;

use bevy::{prelude::*};
use paddle::PaddlePlugin;

const FPS: f32 = 1f32 / 60f32;

const LEFT_WALL: f32 = -450f32;
const RIGHT_WALL: f32 = 450f32;
const TOP_WALL: f32 = -300f32;
const BOTTOM_WALL: f32 = 300f32;

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
        .add_startup_system(setup)
        .add_plugin(PaddlePlugin)
        .run();
}

#[derive(Component)]
struct Ball;

fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

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
    });
}