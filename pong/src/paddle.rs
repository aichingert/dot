use bevy::{prelude::*};

const SPEED: f32 = 600f32;
const LEFT_BOUND: f32 = super::LEFT_WALL + 150.0 / 2.0;
const RIGHT_BOUND: f32 = super::RIGHT_WALL - 150.0 / 2.0;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Paddle {
    id: usize,
}

impl Paddle {
    fn new(id: usize) -> Self {
        Self { 
            id,
         }
    }
}

pub fn paddle_setup(
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

pub fn paddle_movement(
    mut query: Query<(&mut Transform, &mut Paddle)>,
    keyboard_input: ResMut<Input<KeyCode>>,
) {
    let mut first_paddle_direction: f32 = 0.0;
    let mut second_paddle_direction: f32 = 0.0;

    let mut first_paddle: bool = false;
    let mut second_paddle: bool = false;

    if keyboard_input.pressed(KeyCode::A) {
        first_paddle_direction -= 1.0;
        first_paddle = true;
    }

    if keyboard_input.pressed(KeyCode::D) {
        first_paddle_direction += 1.0;
        first_paddle = true;
    }

    if keyboard_input.pressed(KeyCode::Left) {
        second_paddle_direction -= 1.0;
        second_paddle = true;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        second_paddle_direction += 1.0;
        second_paddle = true;
    }

    let mut new_paddle_position: f32 = 0.0;

    query.for_each_mut( | (mut paddle_transform, paddle) | {
        if paddle.id == 1 && first_paddle {
            new_paddle_position = paddle_transform.translation.x + first_paddle_direction * SPEED * super::FPS;
            paddle_transform.translation.x = new_paddle_position.clamp(LEFT_BOUND, RIGHT_BOUND);
        }

        if paddle.id == 2 && second_paddle {
            new_paddle_position = paddle_transform.translation.x + second_paddle_direction * SPEED * super::FPS;
            paddle_transform.translation.x = new_paddle_position.clamp(LEFT_BOUND, RIGHT_BOUND);
        }
    });

}