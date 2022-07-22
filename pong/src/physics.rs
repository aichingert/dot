use bevy::prelude::*;

const WALL_THICKNESS: f32 = 5f32;
const WALL_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

#[derive(Component)]
pub struct Collider;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Default)]
pub struct CollisionEvent;

#[derive(Bundle)]
pub struct WallBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

pub enum WallLocation {
    Right,
    Left,
    Top,
    Bottom,
}

impl WallLocation {
    pub fn position(&self) -> Vec2 {
        match self {
            WallLocation::Right => Vec2::new(super::RIGHT_WALL, 0.0),
            WallLocation::Left => Vec2::new(super::LEFT_WALL, 0.0),
            WallLocation::Top => Vec2::new(0.0, super::TOP_WALL),
            WallLocation::Bottom => Vec2::new(0.0, super::BOTTOM_WALL),
        }
    }

    pub fn size(&self) -> Vec2 {
        let arena_height = super::TOP_WALL - super::BOTTOM_WALL;
        let arena_width = super::RIGHT_WALL - super::LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    pub fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),

                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    query.for_each_mut( | (mut transform, velocity) | {
        transform.translation.x += velocity.x * super::FPS;
        transform.translation.y += velocity.y * super::FPS;
    });
}