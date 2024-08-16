use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<Score>();
}

#[derive(Resource, Default)]
pub struct Score {
    pub current: u32,
    pub high: u32,
    pub previous_high: u32,
}
