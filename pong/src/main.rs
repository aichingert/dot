use bevy::{prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

#[derive(Component)]
struct Paddle;

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
                translation: Vec3::new(0.0, -50.0, 1.0), 
                scale: Vec3::new(50.0, 50.0, 1.0),
                ..default()
            },
        ..default()
    });

    commands
        .spawn()
        .insert(Paddle)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(250.0, 30.0, 0.0),
                translation: Vec3::new(0.0, -250.0, 1.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.9, 0.9, 0.9),
                ..default()
            },
            ..default()
    });

    commands
        .spawn()
        .insert(Paddle)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(250.0, 30.0, 0.0),
                translation: Vec3::new(0.0, 250.0, 1.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.9, 0.9, 0.9),
                ..default()
            },
            ..default()
    });
    //  Vec3::new(30.0, 30.0, 0.0);
}