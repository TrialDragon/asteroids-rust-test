use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{destruction::Destroyed, player::Player, states::GameState, stats::Health};

pub fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(GameState::Loading).load_collection::<HealthPickupAssets>(),
    );
    app.observe(spawn_health_pickup);
    app.add_systems(
        Update,
        (
            health_pickup_tick,
            health_pickup_collision,
            health_pickup_timeout,
            health_pickup_destroyed,
        )
            .chain(),
    );
}

#[derive(AssetCollection, Resource)]
struct HealthPickupAssets {
    #[asset(key = "image.health_pickup")]
    health_pickup: Handle<Image>,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct HealthPickup(Timer);

#[derive(Event, Debug)]
pub struct SpawnHealthPickup {
    position: Vec3,
}

impl SpawnHealthPickup {
    pub fn new(position: Vec3) -> Self {
        Self { position }
    }
}

fn spawn_health_pickup(
    trigger: Trigger<SpawnHealthPickup>,
    mut commands: Commands,
    assets: Res<HealthPickupAssets>,
) {
    const HEALTH_PICKUP_SECS: f32 = 20.;
    let event = trigger.event();

    commands.spawn((
        Name::new("HealthPickup"),
        HealthPickup(Timer::from_seconds(HEALTH_PICKUP_SECS, TimerMode::Once)),
        StateScoped(GameState::Playing),
        Collider::circle(20.),
        SpriteBundle {
            transform: Transform {
                translation: event.position,
                scale: Vec3::new(0.5, 0.5, 1.0),
                ..default()
            },
            texture: assets.health_pickup.clone(),
            ..default()
        },
    ));
}

fn health_pickup_tick(mut query: Query<&mut HealthPickup>, time: Res<Time>) {
    for mut health_pickup in &mut query {
        health_pickup.0.tick(time.delta());
    }
}

fn health_pickup_timeout(
    query: Query<(&HealthPickup, Entity)>,
    mut destroyed_event_writer: EventWriter<Destroyed>,
) {
    for (health_pickup, entity) in &query {
        if health_pickup.0.just_finished() {
            destroyed_event_writer.send(Destroyed(entity));
        }
    }
}

fn health_pickup_collision(
    mut collision_event_reader: EventReader<CollisionStarted>,
    mut destroyed_event_writer: EventWriter<Destroyed>,
    mut player_query: Query<&mut Health, With<Player>>,
    health_pickup_query: Query<(), With<HealthPickup>>,
) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        let mut logic = |first_entity: &Entity, second_entity: &Entity| {
            if !(health_pickup_query.contains(*first_entity)
                && player_query.contains(*second_entity))
            {
                return;
            }

            let Ok(mut health) = player_query.get_mut(*second_entity) else {
                return;
            };

            health.add(1);

            destroyed_event_writer.send(Destroyed(*first_entity));
        };

        logic(entity1, entity2);
        logic(entity2, entity1);
    }
}

fn health_pickup_destroyed(
    mut destroyed_event_reader: EventReader<Destroyed>,
    health_pickup_query: Query<(), With<HealthPickup>>,
    mut commands: Commands,
) {
    for Destroyed(entity) in destroyed_event_reader.read() {
        if health_pickup_query.contains(*entity) {
            commands.entity(*entity).despawn_recursive();
        }
    }
}
