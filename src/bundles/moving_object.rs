use bevy::prelude::*;

use crate::plugins::{
    collision_detection::Collider,
    movement::{Acceleration, Velocity},
    rotation::RotationVelocity,
};

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub rotation_velocity: RotationVelocity,
    pub collider: Collider,
    pub model: SceneBundle,
}
