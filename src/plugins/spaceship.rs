use bevy::prelude::*;

// project internal
use super::{
    collision_detection::{Collider, CollisionDemage},
    movement::{Acceleration, Velocity},
    rotation::RotationVelocity,
    schedule::InGameSet,
};
use crate::{
    bundles::moving_object::MovingObjectBundle, health::Health,
    resources::asset_loader::SceneAssets, state::GameState,
};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const SPACESHIP_COLLIDER_RADIUS: f32 = 7.0;
const HEALTH: f32 = 100.0;
const COLLISION_DEMAGE: f32 = 10.0;

const MISSILE_SPEED: f32 = 20.0;
const MISSILE_FORWARD_SPAWN_SCALAR: f32 = 7.5;
const MISSILE_COLLIDER_RADIUS: f32 = 0.03;
const MISSILE_HEALTH: f32 = 1.0;
const MISSILE_COLLISION_DEMAGE: f32 = 1.0;

#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct SpaceshipMissile;

#[derive(Component, Debug)]
pub struct SpaceshipShield;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship)
            .add_systems(
                Update,
                (
                    spaceship_movement_controls,
                    spaceship_weapon_controls,
                    spaceship_shield_controls,
                )
                    .chain()
                    .in_set(InGameSet::UserInput),
            )
            .add_systems(Update, spaceship_destroyed.in_set(InGameSet::EntityUpdates))
            .add_systems(OnEnter(GameState::GameOver), spawn_spaceship);
    }
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            rotation_velocity: RotationVelocity::new(Vec3::ZERO),
            model: SceneBundle {
                scene: scene_assets.spaceship.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
            collider: Collider::new(SPACESHIP_COLLIDER_RADIUS),
        },
        Name::new("Spaceship"),
        Spaceship,
        Health::new(HEALTH),
        CollisionDemage::new(COLLISION_DEMAGE),
    ));
}

fn spaceship_movement_controls(
    mut query: Query<(&Transform, &mut Velocity, &mut RotationVelocity), With<Spaceship>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let Ok((transform, mut velocity, mut rotation_component)) = query.get_single_mut() else {
        return;
    };
    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    // forward and backward
    if keyboard_input.pressed(KeyCode::W) {
        movement = SPACESHIP_SPEED;
    } else if keyboard_input.pressed(KeyCode::S) {
        movement = -SPACESHIP_SPEED;
    }
    velocity.value = -transform.forward() * movement;

    // turn left and right
    if keyboard_input.pressed(KeyCode::A) {
        rotation = SPACESHIP_ROTATION_SPEED;
    } else if keyboard_input.pressed(KeyCode::D) {
        rotation = -SPACESHIP_ROTATION_SPEED;
    }
    rotation_component.value.y = rotation;

    // roll
    if keyboard_input.pressed(KeyCode::E) {
        roll = SPACESHIP_ROLL_SPEED;
    } else if keyboard_input.pressed(KeyCode::Q) {
        roll = -SPACESHIP_ROLL_SPEED;
    }
    rotation_component.value.z = roll;
}

fn spaceship_weapon_controls(
    mut commands: Commands,
    query: Query<&Transform, With<Spaceship>>,
    keyboard_input: Res<Input<KeyCode>>,
    scene_assets: Res<SceneAssets>,
) {
    let Ok(transform) = query.get_single() else {
        return;
    };
    if keyboard_input.pressed(KeyCode::Space) {
        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity::new(-transform.forward() * MISSILE_SPEED),
                acceleration: Acceleration::new(Vec3::ZERO),
                rotation_velocity: RotationVelocity::new(Vec3::ZERO),
                collider: Collider::new(MISSILE_COLLIDER_RADIUS),
                model: SceneBundle {
                    scene: scene_assets.missiles.clone(),
                    transform: Transform::from_translation(
                        transform.translation + -transform.forward() * MISSILE_FORWARD_SPAWN_SCALAR,
                    ),
                    ..default()
                },
            },
            Name::new("SpaceshipMissile"),
            SpaceshipMissile,
            Health::new(MISSILE_HEALTH),
            CollisionDemage::new(MISSILE_COLLISION_DEMAGE),
        ));
    }
}

fn spaceship_shield_controls(
    mut commands: Commands,
    query: Query<Entity, With<Spaceship>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let Ok(spaceship) = query.get_single() else {
        return;
    };
    if keyboard_input.pressed(KeyCode::Tab) {
        commands.entity(spaceship).insert(SpaceshipShield);
    }
}

fn spaceship_destroyed(
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<(), With<Spaceship>>,
) {
    if query.get_single().is_err() {
        next_state.set(GameState::GameOver);
    }
}
