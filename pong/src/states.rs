use bevy::{
    prelude::*,
    time::FixedTimestep,
};

use crate::physics::*;
use crate::paddle::*;
use crate::ball::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Menu,
    Playing
}

#[derive(Default, Debug)]
pub struct GameResult(pub Option<bool>);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameResult>()
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(paddle_setup)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_run_criteria(FixedTimestep::step(super::FPS as f64))
                    .with_system(check_for_collisions)
                    .with_system(reset_ball)
                    .with_system(paddle_movement.before(check_for_collisions))
                    .with_system(apply_velocity.before(check_for_collisions))
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Playing)
                    .with_system(clean_up_paddles)
            );
    }
}