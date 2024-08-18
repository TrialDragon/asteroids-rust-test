use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    destruction::Destroyed, GameState, BOTTOM_VIEWPORT_EDGE, LEFT_VIEWPORT_EDGE,
    RIGHT_VIEWPORT_EDGE, TOP_VIEWPORT_EDGE, VIEWPORT_HEIGHT, VIEWPORT_WIDTH,
};

pub fn plugin(app: &mut App) {
    // TODO: This could probably be polished
    // to use app sets to fix the amgbiguous
    // system running order, and the bugs
    // that may arise from that ambiguity.
    app.observe(setup_viewport_collider);
    app.add_systems(Update, (movement_wrapping, out_of_bounds_destruction));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct ViewportCollider;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct WrapMovement;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct DestroyOutOfBounds;

#[derive(Event, Debug)]
pub struct SetupViewportCollider;

fn setup_viewport_collider(_: Trigger<SetupViewportCollider>, mut commands: Commands) {
    commands.spawn((
        Name::new("ViewportCollider"),
        ViewportCollider,
        StateScoped(GameState::Playing),
        Collider::rectangle(VIEWPORT_WIDTH, VIEWPORT_HEIGHT),
    ));
}

fn movement_wrapping(
    mut event_reader: EventReader<CollisionEnded>,
    mut wrap_movement_query: Query<&mut Transform, With<WrapMovement>>,
    viewport_collider_query: Query<&Collider, With<ViewportCollider>>,
) {
    for CollisionEnded(entity1, entity2) in event_reader.read() {
        let mut logic = |first_entity: &Entity, second_entity: &Entity| {
            if wrap_movement_query.contains(*first_entity)
                && viewport_collider_query.contains(*second_entity)
            {
                let mut transform = wrap_movement_query.get_mut(*first_entity).unwrap();
                let position = transform.translation;
                if position.x > RIGHT_VIEWPORT_EDGE {
                    transform.translation.x = LEFT_VIEWPORT_EDGE + 1.;
                } else if position.x < LEFT_VIEWPORT_EDGE {
                    transform.translation.x = RIGHT_VIEWPORT_EDGE - 1.;
                }

                if position.y > TOP_VIEWPORT_EDGE {
                    transform.translation.y = BOTTOM_VIEWPORT_EDGE + 1.;
                } else if position.y < BOTTOM_VIEWPORT_EDGE {
                    transform.translation.y = TOP_VIEWPORT_EDGE + 1.;
                }
            }
        };

        logic(entity1, entity2);
        logic(entity2, entity1);
    }
}

fn out_of_bounds_destruction(
    mut event_reader: EventReader<CollisionEnded>,
    mut destroyed_event_writer: EventWriter<Destroyed>,
    mut out_of_bounds_query: Query<&Transform, With<DestroyOutOfBounds>>,
    viewport_collider_query: Query<&Collider, With<ViewportCollider>>,
) {
    for CollisionEnded(entity1, entity2) in event_reader.read() {
        let mut logic = |first_entity: &Entity, second_entity: &Entity| {
            if out_of_bounds_query.contains(*first_entity)
                && viewport_collider_query.contains(*second_entity)
            {
                let position = out_of_bounds_query
                    .get_mut(*first_entity)
                    .unwrap()
                    .translation;
                let out_of_bounds = position.x > RIGHT_VIEWPORT_EDGE
                    || position.x < LEFT_VIEWPORT_EDGE
                    || position.y > TOP_VIEWPORT_EDGE
                    || position.y < BOTTOM_VIEWPORT_EDGE;
                if out_of_bounds {
                    destroyed_event_writer.send(Destroyed(*first_entity));
                }
            }
        };

        logic(entity1, entity2);
        logic(entity2, entity1);
    }
}
