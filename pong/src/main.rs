mod paddle;
mod scoreboard;
mod physics;
mod ball;
mod states;

use bevy::prelude::*;
use paddle::*;
use scoreboard::ScoreboardPlugin;
use physics::*;
use states::*;
use ball::BallPlugin;

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
        .add_state(GameState::Playing)
        .add_plugins(DefaultPlugins)
        .add_plugin(ScoreboardPlugin)
        .add_startup_system(paddle_setup)
        .add_event::<CollisionEvent>()
        .add_plugin(GamePlugin)
        .add_plugin(BallPlugin)
        .run();
}