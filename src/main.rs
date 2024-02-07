mod bundles;
mod plugins;
mod resources;

use bevy::prelude::*;
use plugins::{
    // asteroids::AsteroidPlugin,
    camera::CameraPlugin,
    debug::DebugPlugin,
    movement::MovementPlugin,
    rotation::RotationPlugin,
    spaceship::SpaceshipPlugin,
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
        // .add_plugins(AsteroidPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(RotationPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(DebugPlugin)
        .run();
}
