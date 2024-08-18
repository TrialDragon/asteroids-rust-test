use bevy::prelude::*;

use crate::{
    asteroid::{SetupAsteroidSpawners, SpawnAsteroids},
    player::SpawnPlayer,
    viewport_bound::SetupViewportCollider,
    GameState,
};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), setup_play_area);
}

fn setup_play_area(mut commands: Commands) {
    // setup viewport collider
    commands.trigger(SetupViewportCollider);
    // player
    commands.trigger(SpawnPlayer);
    // setup asteroid_spawners
    commands.trigger(SetupAsteroidSpawners);
    // spawn initial asteroids
    commands.trigger(SpawnAsteroids::new(5));
}
