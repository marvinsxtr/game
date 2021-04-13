use bevy::prelude::*;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(player_setup.system())
           .add_system(player_movement.system());
    }
}
pub struct Player {
    pub speed: f32,
}

fn player_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.2, 0.5, 0.5).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        sprite: Sprite::new(Vec2::new(30.0, 30.0)),
        ..Default::default()
    })
    .insert(Player { speed: 500.0 });
}

fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    for (player, mut transform) in &mut query.iter_mut() {

        let arrow_keys: Vec<KeyCode> = [
            KeyCode::Left, 
            KeyCode::Right, 
            KeyCode::Up, 
            KeyCode::Down
        ].into();

        for key_code in arrow_keys {
            if !keyboard_input.pressed(key_code) { continue };
            
            transform.translation.x += time.delta_seconds() * player.speed * 
                match key_code {
                    KeyCode::Left => -1.0,
                    KeyCode::Right => 1.0,
                    _ => { 0f32 }
                };

            transform.translation.y += time.delta_seconds() * player.speed * 
                match key_code {
                    KeyCode::Up => 1.0,
                    KeyCode::Down => -1.0,
                    _ => { 0f32 }
                };
        }
    }
}
