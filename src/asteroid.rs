use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use rand::{seq::IteratorRandom, thread_rng, Rng};

use crate::GameState;

pub fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(GameState::Loading)
            .load_collection::<AsteroidAssets>(),
    );
    app.insert_resource(AsteroidID(0));
    app.observe(spawn_asteroid);
    app.add_systems(OnEnter(GameState::Playing), (setup_asteroid_spawners, spawn_asteroids).chain());
}

#[derive(AssetCollection, Resource)]
struct AsteroidAssets {
    #[asset(key = "image.basic_asteroid")]
    basic_asteroid: Handle<Image>
}

#[derive(Resource)]
struct AsteroidID(pub usize);

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Asteroid {
    id: usize,
    direction: Vec3,
}

#[derive(Debug)]
enum AsteroidKind {
    Basic,
    SmallBasic,
    Advanced,
    SmallAdvanced,
}

impl AsteroidKind {
    // TODO: use this for being shot at.
    pub fn _get_health(&self) -> u32 {
        match self {
            AsteroidKind::Basic | AsteroidKind::SmallBasic => 1,
            AsteroidKind::Advanced | AsteroidKind::SmallAdvanced => 3,
        }
    }

    fn get_texture(&self,assets: Res<AsteroidAssets>) -> Handle<Image> {
        match self {
            AsteroidKind::Basic => assets.basic_asteroid.clone(),
            AsteroidKind::Advanced => todo!(),
            AsteroidKind::SmallBasic => todo!(),
            AsteroidKind::SmallAdvanced => todo!(),
        }
    }
}

#[derive(Event,Debug)]
pub struct SpawnAsteroid {
    kind: AsteroidKind,
    position: Vec3,
    direction: Vec3,
}

impl SpawnAsteroid {
    fn new(
        kind: AsteroidKind,
        position: Vec3,
        direction: Vec3
    ) -> Self {
        Self {
            kind,
            position,
            direction
        }
    }
}

fn spawn_asteroid(trigger: Trigger<SpawnAsteroid>, mut commands: Commands, assets: Res<AsteroidAssets>, mut asteroid_id: ResMut<AsteroidID>) {
    let event = trigger.event();

    commands.spawn((
        Name::new("Basic Asteroid"),
        StateScoped(GameState::Playing),
        Asteroid {
            id: asteroid_id.0,
            direction: event.direction,
        },
        SpriteBundle {
            transform: Transform::from_translation(event.position),
            texture: event.kind.get_texture(assets),
            ..default()
        },
    ));

    // Increment the ID so that the next spawned
    // asteroid has its own unique ID.
    //
    // We use a wrapping (overflowing) add
    // because on the unlikely off chance that
    // the ID's get that far we can presume it's
    // safe to go back to the start.
    // If an asteroid of ID 0, or some other low number,
    // still exists at that point, then something
    // has gone terribly wrong with de-spawning
    // or the user's PC is on fire from simply
    // spawning too many entities at once.
    asteroid_id.0 = asteroid_id.0.wrapping_add(1);
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
            transform_bundle: TransformBundle::from_transform(Transform::from_translation(position)),
            ..default()
        }
    }
}

fn setup_asteroid_spawners(mut commands: Commands) {
    const OFFSET: f32 = 40.;

    const RIGHT_VIEWPORT_EDGE: f32 = 640. + OFFSET;
    const LEFT_VIEWPORT_EDGE: f32 = -640. - OFFSET;
    const TOP_VIEWPORT_EDGE: f32 = 360. + OFFSET;
    const BOTTOM_VIEWPORT_EDGE: f32 = -360. - OFFSET;

    let spawner_points: Vec<Vec3> = vec![
        // Corners.
        Vec3::new(RIGHT_VIEWPORT_EDGE, TOP_VIEWPORT_EDGE, 0.0),
        Vec3::new(RIGHT_VIEWPORT_EDGE, BOTTOM_VIEWPORT_EDGE, 0.0),
        Vec3::new(LEFT_VIEWPORT_EDGE, TOP_VIEWPORT_EDGE, 0.0),
        Vec3::new(LEFT_VIEWPORT_EDGE, BOTTOM_VIEWPORT_EDGE, 0.0),
        // Left and right edges.
        Vec3::new(LEFT_VIEWPORT_EDGE, 0.0, 0.0),
        Vec3::new(RIGHT_VIEWPORT_EDGE, 0.0, 0.0),
        // Top edge.
        Vec3::new(LEFT_VIEWPORT_EDGE / 2., TOP_VIEWPORT_EDGE, 0.0),
        Vec3::new(RIGHT_VIEWPORT_EDGE / 2., TOP_VIEWPORT_EDGE, 0.0),
        // Bottom edge.
        Vec3::new(LEFT_VIEWPORT_EDGE / 2., BOTTOM_VIEWPORT_EDGE, 0.0),
        Vec3::new(RIGHT_VIEWPORT_EDGE / 2., BOTTOM_VIEWPORT_EDGE, 0.0),

    ];

    for spawner_point in spawner_points {
        let mut rng = thread_rng();
        let random_angle: f32 = rng.gen_range(-1.5..=1.5);
        let mut direction = Transform::from_translation((Vec3::ZERO - spawner_point).normalize_or_zero());
        direction.rotate_local_z(random_angle);

        // Direction from A to B is B - A;
        // we then normalize it to make it a unit vector.
        commands.spawn(AsteroidSpawnerBundle::new(spawner_point, direction.translation));
    }
}

fn spawn_asteroids(query: Query<(&AsteroidSpawner, &Transform)>, mut commands: Commands) {
    let mut spawned_asteroids: Vec<Vec3> = vec![];
    let mut rng = rand::thread_rng();

    while spawned_asteroids.len() < 5 {
        let (spawner, transform) = query.iter().choose(&mut rng).unwrap();

        if spawned_asteroids.contains(&transform.translation) {
            continue;
        }

        // TODO: Expand to use multiple types of
        // asteroids.
        commands.trigger(SpawnAsteroid::new(
            AsteroidKind::Basic,
            transform.translation,
            spawner.normal_direction,
        ));

        spawned_asteroids.push(transform.translation);
    }
}
