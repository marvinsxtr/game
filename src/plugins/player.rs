use bevy::{prelude::*, render::camera::Camera};
use heron::prelude::*;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(player_setup.system())
           .add_system(player_movement.system())
           .add_system(player_weapon.system());
    }
}

pub struct Player {
    pub speed: f32,
    pub direction: Quat,
}

fn player_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.2, 0.5, 0.5).into()),
        transform: Transform::from_xyz(0., 0., 0.),
        sprite: Sprite::new(Vec2::new(30., 30.)),
        ..Default::default()
    })
    .insert(Player { speed: 500. , direction: Quat::IDENTITY});
}

fn player_movement(
    time: Res<Time>,
    windows: Res<Windows>,
    cam_query: Query<&Camera>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform)>,
    mut evr_cursor: EventReader<CursorMoved>,
) {
    // Get the camera and the player
    let camera: &Camera = cam_query.single().unwrap();
    let (mut player, mut transform) = query.single_mut().unwrap();

    // Get the screen position of the player
    let player_screen_pos: Vec2 = camera
        .world_to_screen(&windows, &GlobalTransform::identity(), transform.translation)
        .unwrap();

    // Get change of cursor
    match evr_cursor.iter().last() {
        Some(cursor_screen_pos) => {
            // Calculate the angle of the player towards the cursor
            let a: f32 = player_screen_pos.x - cursor_screen_pos.position.x;
            let b: f32 = player_screen_pos.y - cursor_screen_pos.position.y;

            let angle: f32 = b.atan2(a);
            
            // Adjust player direction
            player.direction = Quat::from_rotation_z(angle);
        },
        _ => (),
    };

    // Handle player movement
    let arrow_keys: Vec<KeyCode> = [
        KeyCode::A, 
        KeyCode::D, 
        KeyCode::W, 
        KeyCode::S
    ].into();

    for key_code in arrow_keys {
        if !keyboard_input.pressed(key_code) { continue };
        
        transform.translation.x += time.delta_seconds() * player.speed * 
            match key_code {
                KeyCode::A => -1.,
                KeyCode::D => 1.,
                _ => { 0. }
            };

        transform.translation.y += time.delta_seconds() * player.speed * 
            match key_code {
                KeyCode::W => 1.,
                KeyCode::S => -1.,
                _ => { 0. }
            };
    }
}

fn player_weapon(
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<(&Player, &Transform)>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Get the player
    let (player, transform) = query.single().unwrap();

    // Shoot when space is pressed
    if keyboard_input.pressed(KeyCode::Space) {

        // Spawn the projectile
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(Color::rgb(0.7, 0., 0.).into()),
                transform: Transform {
                    translation: transform.translation,
                    rotation: player.direction,
                    ..Default::default()
                },
                sprite: Sprite::new(Vec2::new(50., 10.)),
                ..Default::default()
            })
            .insert(GlobalTransform::identity())
            .insert(Body::Sphere { radius: 10.0 })
            .insert(BodyType::Kinematic)
            .insert(Velocity::from(-player.direction.mul_vec3(Vec3::X) * 1000.));
    }
}