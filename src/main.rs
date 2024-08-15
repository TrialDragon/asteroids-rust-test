use avian2d::prelude::*;
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_asset_loader::prelude::*;
use bevy_transform_interpolation::*;
use leafwing_input_manager::prelude::*;

use game_library::{player, GameState, Action};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(TransformInterpolationPlugin::default())
        .add_plugins(InputManagerPlugin::<Action>::default())
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

fn setup_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::Fixed {
        width: 1280.,
        height: 720.,
    };
    commands.spawn((
        Name::new("Camera"),
        camera_bundle,
        IsDefaultUiCamera,
    ));
}
