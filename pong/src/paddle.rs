use bevy::prelude::*;

#[derive(Component)]
pub struct Paddle {
    id: usize,
}

impl Paddle {
    fn new(id: usize) -> Self {
        Self { 
            id
         }
    }
}

pub struct PaddlePlugin; 

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(
    mut commands: Commands,
) {
    commands
    .spawn()
    .insert(Paddle::new(1))
    .insert_bundle(SpriteBundle {
        transform: Transform {
            scale: Vec3::new(150.0, 30.0, 0.0),
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
        .insert(Paddle::new(2))
        .insert_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(150.0, 30.0, 0.0),
                translation: Vec3::new(0.0, 250.0, 1.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.9, 0.9, 0.9),
                ..default()
            },
            ..default()
    });
}