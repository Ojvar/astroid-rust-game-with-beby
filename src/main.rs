use asset_loader::AssetLoaderPlugin;
use asteroid::AsteroidPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use collision_detection::CollisionDetectionPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

mod asset_loader;
mod asteroid;
mod camera;
mod collision_detection;
mod debug;
mod movement;
mod spaceship;

fn main() {
    App::new()
        // Bevy plugins
        .insert_resource(ClearColor(Color::rgb(0.1, 0., 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(DefaultPlugins)
        // User plugins
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(CollisionDetectionPlugin)
        // .add_plugins(DebugPlugin)
        .run();
}
