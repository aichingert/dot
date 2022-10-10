mod paddle;
mod scoreboard;
mod physics;
mod ball;
mod states;
mod menu;

use bevy::prelude::*;
use scoreboard::ScoreboardPlugin;
use states::*;
use ball::BallPlugin;
use menu::MenuPlugin;

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
        .add_state(GameState::Menu)
        .add_plugins(DefaultPlugins)
        .add_plugin(ScoreboardPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(BallPlugin)
        .add_plugin(MenuPlugin)
        .run();
}