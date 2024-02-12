use bevy::prelude::*;

use super::{collision_detection::Collider, schedule::InGameSet};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (print_position, print_collisions)
                .chain()
                .after(InGameSet::EntityUpdates),
        );
    }
}

fn print_position(query: Query<(Entity, &Transform)>) {
    for (entity, transform) in &query {
        info!("Entity {:?} is at transform {:?}", entity, transform);
    }
}

fn print_collisions(query: Query<(Entity, &Name, &Collider)>) {
    for (entity, name, collider) in &query {
        if collider.colliding_entities.len() > 0 {
            let collided_entities_name_list: Vec<_> = collider
                .colliding_entities
                .iter()
                .copied()
                .filter_map(|e| query.get(e).ok().map(|(_, name, _)| (name, e)))
                .collect();
            info!(
                "Entity [{:?}:{:?}] collision with {:?}",
                name, entity, collided_entities_name_list
            );
        }
    }
}
