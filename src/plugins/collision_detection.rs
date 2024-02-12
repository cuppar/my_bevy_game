use bevy::{
    prelude::*,
    utils::hashbrown::{HashMap, HashSet},
};

use super::{asteroids::Asteroid, schedule::InGameSet, spaceship::Spaceship};

#[derive(Component, Debug, Default)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entities: vec![],
        }
    }
}

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            collision_detection.in_set(InGameSet::CollisionDetection),
        )
        .add_systems(
            Update,
            (
                handle_collisions::<Asteroid>,
                handle_collisions::<Spaceship>,
            )
                .in_set(InGameSet::DespawnEntities),
        );
    }
}

fn collision_detection(mut query: Query<(Entity, &mut Collider, &GlobalTransform)>) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    // detection argo 1
    // for (entity, collider, global_transform) in &query {
    //     for (other, other_collider, other_global_transform) in
    //         query.iter().filter(|&(e, ..)| e != entity)
    //     {
    //         let distance = global_transform
    //             .translation()
    //             .distance(other_global_transform.translation());
    //         if distance < collider.radius + other_collider.radius {
    //             colliding_entities
    //                 .entry(entity)
    //                 .or_insert_with(Vec::new)
    //                 .push(other);
    //         }
    //     }
    // } // end of detection argo 1

    // detection argo 2
    let mut checked_entities: HashSet<Entity> = HashSet::new();
    for (entity, collider, global_transform) in &query {
        for (other, other_collider, other_global_transform) in
            query.iter().filter(|&(e, ..)| e != entity)
        {
            if checked_entities.contains(&other) {
                continue;
            }
            let distance = global_transform
                .translation()
                .distance(other_global_transform.translation());
            if distance < collider.radius + other_collider.radius {
                colliding_entities
                    .entry(entity)
                    .or_insert_with(Vec::new)
                    .push(other);
                colliding_entities
                    .entry(other)
                    .or_insert_with(Vec::new)
                    .push(entity);
            }
        }
        checked_entities.insert(entity);
    } // end of detection argo 2

    for (entity, mut collider, _) in &mut query {
        collider.colliding_entities.clear();
        if let Some(collisions) = colliding_entities.get(&entity) {
            collider
                .colliding_entities
                .extend(collisions.iter().copied())
        }
    }
}

fn handle_collisions<T: Component>(
    mut commands: Commands,
    query: Query<(Entity, &Collider), With<T>>,
) {
    for (entity, collider) in query.iter() {
        for &collided_entity in collider.colliding_entities.iter() {
            if query.get(collided_entity).is_ok() {
                continue;
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}
