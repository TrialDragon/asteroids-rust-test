use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<Score>();
}

#[derive(Resource, Default)]
pub struct Score {
    current: u32,
    high: u32,
    previous_high: u32,
}
