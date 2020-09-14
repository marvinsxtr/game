use bevy::prelude::*;

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(HelloPlugin)
        .run();
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(HelloWorldTimer(Timer::from_seconds(2.0, true)))
            .add_system(hello_world.system());
    }
}

pub struct HelloWorldTimer(Timer);

fn hello_world(time: Res<Time>, mut timer: ResMut<HelloWorldTimer>) {
    timer.0.tick(time.delta_seconds);

    if timer.0.finished {
        println!("Hello world!");
    }
}
