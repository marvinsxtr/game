use bevy::prelude::*;
use player::{Collider, Player, PlayerPlugin};
mod player;

fn main() {
    App::build()
        .add_default_plugins()
        .add_resource(ClearColor(Color::rgb(0.7, 0.7, 0.7)))
        .add_startup_system(setup.system())
        .add_plugin(PlayerPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    _asset_server: Res<AssetServer>,
) {
    // Add the game's entities to the world
    commands
        // Camera
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        // Player
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.2, 0.5, 0.5).into()),
            translation: Translation(Vec3::new(0.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .with(Player { speed: 500.0 })
        .with(Collider::Solid);
}
