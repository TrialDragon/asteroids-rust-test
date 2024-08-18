use bevy::prelude::*;
use rand::{seq::IteratorRandom, thread_rng, Rng};

use crate::{states::GameState, BOTTOM_VIEWPORT_EDGE, LEFT_VIEWPORT_EDGE, RIGHT_VIEWPORT_EDGE, TOP_VIEWPORT_EDGE};

use super::{AsteroidID, AsteroidKind, SpawnAsteroid};

pub(super) fn plugin(app: &mut App) {
    app.observe(setup_asteroid_spawners);
    app.observe(spawn_asteroids);
}

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
struct AsteroidSpawner {
    normal_direction: Vec3,
}

impl AsteroidSpawner {
    pub fn new(direction: Vec3) -> Self {
        Self {
            normal_direction: direction,
        }
    }
}

#[derive(Bundle)]
struct AsteroidSpawnerBundle {
    name: Name,
    state_scoped: StateScoped<GameState>,
    spawner: AsteroidSpawner,
    transform_bundle: TransformBundle,
}

impl Default for AsteroidSpawnerBundle {
    fn default() -> Self {
        Self {
            name: Name::new("AsteroidSpawner"),
            state_scoped: StateScoped(GameState::Playing),
            spawner: AsteroidSpawner::new(Vec3::ZERO),
            transform_bundle: TransformBundle::default(),
        }
    }
}

impl AsteroidSpawnerBundle {
    pub fn new(position: Vec3, direction: Vec3) -> Self {
        Self {
            spawner: AsteroidSpawner::new(direction),
            transform_bundle: TransformBundle::from_transform(Transform::from_translation(
                position,
            )),
            ..default()
        }
    }
}

#[derive(Event, Debug)]
pub struct SetupAsteroidSpawners;

fn setup_asteroid_spawners(_: Trigger<SetupAsteroidSpawners>, mut commands: Commands) {
    const OFFSET: f32 = 40.;

    const OFFSET_RIGHT_VIEWPORT_EDGE: f32 = RIGHT_VIEWPORT_EDGE + OFFSET;
    const OFFSET_LEFT_VIEWPORT_EDGE: f32 = LEFT_VIEWPORT_EDGE - OFFSET;
    const OFFSET_TOP_VIEWPORT_EDGE: f32 = TOP_VIEWPORT_EDGE + OFFSET;
    const OFFSET_BOTTOM_VIEWPORT_EDGE: f32 = BOTTOM_VIEWPORT_EDGE - OFFSET;

    let spawner_points: Vec<Vec3> = vec![
        // Corners.
        Vec3::new(OFFSET_RIGHT_VIEWPORT_EDGE, OFFSET_TOP_VIEWPORT_EDGE, 0.0),
        Vec3::new(OFFSET_RIGHT_VIEWPORT_EDGE, OFFSET_BOTTOM_VIEWPORT_EDGE, 0.0),
        Vec3::new(OFFSET_LEFT_VIEWPORT_EDGE, OFFSET_TOP_VIEWPORT_EDGE, 0.0),
        Vec3::new(OFFSET_LEFT_VIEWPORT_EDGE, OFFSET_BOTTOM_VIEWPORT_EDGE, 0.0),
        // Left and right edges.
        Vec3::new(OFFSET_LEFT_VIEWPORT_EDGE, 0.0, 0.0),
        Vec3::new(OFFSET_RIGHT_VIEWPORT_EDGE, 0.0, 0.0),
        // Top edge.
        Vec3::new(
            OFFSET_LEFT_VIEWPORT_EDGE / 2.,
            OFFSET_TOP_VIEWPORT_EDGE,
            0.0,
        ),
        Vec3::new(
            OFFSET_RIGHT_VIEWPORT_EDGE / 2.,
            OFFSET_TOP_VIEWPORT_EDGE,
            0.0,
        ),
        // Bottom edge.
        Vec3::new(
            OFFSET_LEFT_VIEWPORT_EDGE / 2.,
            OFFSET_BOTTOM_VIEWPORT_EDGE,
            0.0,
        ),
        Vec3::new(
            OFFSET_RIGHT_VIEWPORT_EDGE / 2.,
            OFFSET_BOTTOM_VIEWPORT_EDGE,
            0.0,
        ),
    ];

    for spawner_point in spawner_points {
        let mut rng = thread_rng();
        let target: Vec3 = Vec3::new(
            rng.gen_range(-320.0..=320.0),
            rng.gen_range(-130.0..=130.0),
            0.0,
        );
        let direction = Transform::from_translation((target - spawner_point).normalize_or_zero());

        // Direction from A to B is B - A;
        // we then normalize it to make it a unit vector.
        commands.spawn(AsteroidSpawnerBundle::new(
            spawner_point,
            direction.translation,
        ));
    }
}

#[derive(Event, Debug)]
pub struct SpawnAsteroids {
    amount: u16,
}

impl SpawnAsteroids {
    pub fn new(amount: u16) -> Self {
        Self { amount }
    }
}

fn spawn_asteroids(
    trigger: Trigger<SpawnAsteroids>,
    query: Query<(&AsteroidSpawner, &Transform)>,
    mut commands: Commands,
    mut asteroid_id: ResMut<AsteroidID>,
) {
    let mut spawned_asteroids: Vec<Vec3> = vec![];
    let mut rng = rand::thread_rng();

    while spawned_asteroids.len() < trigger.event().amount as usize {
        let (spawner, transform) = query.iter().choose(&mut rng).unwrap();

        if spawned_asteroids.contains(&transform.translation) {
            continue;
        }

        let spawn_basic: bool = rng.gen();
        commands.trigger(SpawnAsteroid::new(
            if spawn_basic {
                AsteroidKind::Basic
            } else {
                AsteroidKind::Advanced
            },
            *transform,
            spawner.normal_direction,
            asteroid_id.get(),
        ));

        spawned_asteroids.push(transform.translation);
    }
}
