use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(player_movement.system());
    }
}

pub struct Player {
    pub speed: f32,
}

pub enum Collider {
    Solid,
    // Scorable,
}

fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Translation)>,
) {
    for (player, mut translation) in &mut query.iter() {
        if keyboard_input.pressed(KeyCode::Left) {
            *translation.0.x_mut() += time.delta_seconds * -1.0 * player.speed;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            *translation.0.x_mut() += time.delta_seconds * 1.0 * player.speed;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            *translation.0.y_mut() += time.delta_seconds * 1.0 * player.speed;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            *translation.0.y_mut() += time.delta_seconds * -1.0 * player.speed;
        }
    }
}
