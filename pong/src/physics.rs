use bevy::prelude::*;

#[derive(Component)]
pub struct Collider;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Default)]
pub struct CollisionEvent;

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    query.for_each_mut( | (mut transform, velocity) | {
        transform.translation.x += velocity.x * super::FPS;
        transform.translation.y += velocity.y * super::FPS;
    });
}