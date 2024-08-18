use bevy::prelude::*;

use crate::{
    player::SpawnPlayer, stats::Score, viewport_bound::SetupViewportCollider
};
use crate::asteroid::asteroid_spawner::{SetupAsteroidSpawners, SpawnAsteroids};

use super::GameState;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), setup_play_area);
}

fn setup_play_area(mut commands: Commands, mut score: ResMut<Score>) {
    // setup viewport collider
    commands.trigger(SetupViewportCollider);
    // player
    commands.trigger(SpawnPlayer);
    // setup asteroid_spawners
    commands.trigger(SetupAsteroidSpawners);
    // spawn initial asteroids
    commands.trigger(SpawnAsteroids::new(5));
    // reset the score
    score.0 = 0;
}
