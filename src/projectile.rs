use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_transform_interpolation::*;

use crate::{stats::LinearAcceleration, GameState};

pub fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(GameState::Loading).load_collection::<ProjectileAssets>(),
    );
    app.observe(spawn_projectile);
    app.add_systems(FixedUpdate, move_projectile);
}

#[derive(AssetCollection, Resource)]
struct ProjectileAssets {
    #[asset(key = "image.projectile_sprite")]
    projectile_sprite: Handle<Image>,
}

#[derive(Event, Debug)]
pub struct SpawnProjectile {
    position: Vec3,
    rotation: Quat,
}

impl SpawnProjectile {
    pub fn new(position: Vec3, rotation: Quat) -> Self {
        Self { position, rotation }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Projectile;

fn spawn_projectile(
    trigger: Trigger<SpawnProjectile>,
    mut commands: Commands,
    assets: Res<ProjectileAssets>,
) {
    let event = trigger.event();
    commands.spawn((
        Name::new("Projectile"),
        Projectile,
        StateScoped(GameState::Playing),
        RigidBody::Kinematic,
        Collider::capsule(0.5, 3.),
        DebugRender::default(),
        LinearAcceleration(8000.),
        TranslationInterpolation,
        SpriteBundle {
            transform: Transform {
                translation: event.position,
                rotation: event.rotation,
                ..default()
            },
            texture: assets.projectile_sprite.clone(),
            ..default()
        },
    ));
}

fn move_projectile(
    mut query: Query<(&mut LinearVelocity, &Transform, &LinearAcceleration), With<Projectile>>,
    time: Res<Time<Fixed>>,
) {
    const MAX_PROJECTILE_SPEED: f32 = 1000.0;

    for (mut linear_velocity, transform, linear_acceleration) in &mut query {
        let direction = (transform.rotation * Vec3::Y).xy().normalize_or_zero();
        linear_velocity.0 = linear_velocity.0.move_towards(
            direction * MAX_PROJECTILE_SPEED,
            linear_acceleration.0 * time.delta_seconds(),
        );
    }
}
