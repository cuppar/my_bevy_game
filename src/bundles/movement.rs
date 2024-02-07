use bevy::prelude::*;

use crate::plugins::{
    movement::{Acceleration, Velocity},
    rotation::Rotation,
};

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub rotation: Rotation,
    pub model: SceneBundle,
}
