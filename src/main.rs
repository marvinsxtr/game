use bevy::prelude::*;
use keyboard_plugin::KeyboardInputPlugin;

mod keyboard_plugin;

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(KeyboardInputPlugin)
        .run();
}
