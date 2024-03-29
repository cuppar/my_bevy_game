use bevy::prelude::*;

use super::schedule::InGameSet;

#[derive(Component, Debug)]
pub struct RotationVelocity {
    pub value: Vec3,
}

impl RotationVelocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

pub struct RotationPlugin;

impl Plugin for RotationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_rotation.in_set(InGameSet::EntityUpdates));
    }
}

fn update_rotation(mut query: Query<(&RotationVelocity, &mut Transform)>, time: Res<Time>) {
    for (rotation, mut transform) in &mut query {
        transform.rotate_local_x(rotation.value.x * time.delta_seconds());
        transform.rotate_local_y(rotation.value.y * time.delta_seconds());
        transform.rotate_local_z(rotation.value.z * time.delta_seconds());
    }
}
