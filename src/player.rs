use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use avian2d::prelude::*;

use crate::GameState;

pub fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(GameState::Loading)
            .load_collection::<PlayerAssets>(),
    );
    app.add_systems(OnEnter(GameState::Playing), spawn_player);
    app.add_systems(FixedUpdate, move_player);
    app.add_systems(Update, player_input);
}

#[derive(AssetCollection, Resource)]
struct PlayerAssets {
    #[asset(key = "image.player_sprite")]
    sprite: Handle<Image>
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Player;

fn spawn_player(mut commands: Commands, assets: Res<PlayerAssets>) {
    commands.spawn((
        Name::new("Player"),
        Player,
        RigidBody::Kinematic,
        Collider::circle(1.),
        SpriteBundle {
            texture: assets.sprite.clone(),
            ..default()
        },
    ));
}

fn move_player() {}

fn player_input() {}


