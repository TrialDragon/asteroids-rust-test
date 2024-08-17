use avian2d::prelude::*;
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_transform_interpolation::*;
use leafwing_input_manager::prelude::*;

use game_library::{
    asteroid, destruction, game_over, player, projectile, score, stats, viewport_bound, Action, GameState, VIEWPORT_HEIGHT, VIEWPORT_WIDTH
};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(PhysicsPlugins::default());
    app.add_plugins(TransformInterpolationPlugin::default());
    app.add_plugins(InputManagerPlugin::<Action>::default());
    app.init_state::<GameState>();
    app.enable_state_scoped_entities::<GameState>();
    app.add_loading_state(
        LoadingState::new(GameState::Loading)
            .continue_to_state(GameState::Playing)
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>("game.assets.ron"),
    );
    app.add_plugins(asteroid::plugin);
    app.add_plugins(destruction::plugin);
    app.add_plugins(game_over::plugin);
    app.add_plugins(player::plugin);
    app.add_plugins(projectile::plugin);
    app.add_plugins(score::plugin);
    app.add_plugins(stats::plugin);
    app.add_plugins(viewport_bound::plugin);
    app.add_systems(Startup, setup_camera);
    app.insert_resource(ClearColor(Color::srgb(0., 0., 0.)));

    // Development plugins, systems, et cetera.
    #[cfg(feature = "dev")]
    {
        app.add_plugins(PhysicsDebugPlugin::default());
        app.add_plugins(WorldInspectorPlugin::new());
    }

    app.run();
}

fn setup_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::Fixed {
        width: VIEWPORT_WIDTH,
        height: VIEWPORT_HEIGHT,
    };
    commands.spawn((Name::new("Camera"), camera_bundle, IsDefaultUiCamera));
}
