use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use game_library::player;
use game_library::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading)
            .continue_to_state(GameState::Playing)
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                "game.assets.ron",
            ),
        )
        .add_plugins(player::plugin)
        .add_systems(Startup, setup_camera)
        .run();
}

// WARN: This should be removed later on;
// it exists only for early testing reasons.
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
