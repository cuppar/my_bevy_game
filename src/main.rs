mod bundles;
mod components;
mod plugins;
mod resources;

// third party
use bevy::prelude::*;

// project internal
use plugins::{
    asteroids::AsteroidPlugin, camera::CameraPlugin, collision_detection::CollisionDetectionPlugin,
    debug::DebugPlugin, despawn::DespawnPlugin, movement::MovementPlugin, rotation::RotationPlugin,
    schedule::SchedulePlugin, spaceship::SpaceshipPlugin,
};
use resources::asset_loader::AssetLoaderPlugin;

fn main() {
    App::new()
        // Bevy built-ins.
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(DefaultPlugins)
        // User configured plugins.
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(RotationPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(SchedulePlugin)
        // .add_plugins(DebugPlugin)
        .run();
}
