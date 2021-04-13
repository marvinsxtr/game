use bevy:: {
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use plugins::player::PlayerPlugin;
mod plugins {
    pub mod player;
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup.system())
        .insert_resource(ClearColor(Color::rgb(0.7, 0.7, 0.7)))
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::INFO,
            ..Default::default()
        })
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
