use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{destruction::Destroyed, GameState};

pub fn plugin(app: &mut App) {
    // TODO: This could probably be polished
    // to use app sets to fix the amgbiguous
    // system running order, and the bugs
    // that may arise from that ambiguity.
    app.add_systems(OnEnter(GameState::Playing), setup_viewport_collider);
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

fn setup_viewport_collider(mut commands: Commands) {
    commands.spawn((
        Name::new("ViewportCollider"),
        ViewportCollider,
        StateScoped(GameState::Playing),
        // TODO: refactor the size out
        // into a `lib.rs` constant.
        Collider::rectangle(1280., 720.),
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
                // TODO: Viewport size constant refactor.
                if position.x > 640. {
                    transform.translation.x = -639.;
                } else if position.x < -640. {
                    transform.translation.x = 639.;
                }

                if position.y > 360. {
                    transform.translation.y = -359.;
                } else if position.y < -360. {
                    transform.translation.y = 359.;
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
                // TODO: Viewport size constant refactor.
                let out_of_bounds = position.x > 640.
                    || position.x < -640.
                    || position.y > 360.
                    || position.y < -360.;
                if out_of_bounds {
                    destroyed_event_writer.send(Destroyed(*first_entity));
                }
            }
        };

        logic(entity1, entity2);
        logic(entity2, entity1);
    }
}
