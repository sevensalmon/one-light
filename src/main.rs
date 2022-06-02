use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;
#[cfg(feature = "debug")]
use bevy_flycam::{PlayerPlugin, MovementSettings};


fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "one light".to_string(),
        ..Default::default()
    });

    app.add_plugins(DefaultPlugins);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());
    #[cfg(feature = "debug")]
    app.add_plugin(PlayerPlugin);
    #[cfg(feature = "debug")]
    app.insert_resource(MovementSettings {
        sensitivity: 0.00008,
        speed: 8.0,
    });


    #[cfg(not(feature = "debug"))]
    app.add_startup_system(setup_camera);

    app.add_startup_system(setup_character);
    app.add_startup_system(setup_environment);
    app.add_startup_system(setup_light);

    app.add_system(player_movement);

    app.run();
}

#[derive(Component)]
struct Player;

fn setup_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ) {
    let character = asset_server.load("humans/basic/export/block-person.gltf#Scene0");
    commands.spawn_bundle((
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::identity(),
        Name::new("Human")
    )).with_children(|parent| {
        parent.spawn_scene(character);
    })
    .insert(Player);

}

fn setup_environment(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ) {
    let terrain = asset_server.load("terrain/prototype-ground/export/prototype-ground.glb#Scene0");
    commands.spawn_bundle((
            Transform::from_xyz(0.0, 0.0, 0.0),
            GlobalTransform::identity(),
            Name::new("Terrain")
            )).with_children(|parent| {
        parent.spawn_scene(terrain);
    });

}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 8.0))
            .looking_at(Vec3::default(), Vec3::Y),
        orthographic_projection: OrthographicProjection {
            scale: 0.01,
            ..Default::default()
        },
        ..OrthographicCameraBundle::new_3d()
    });
}

fn setup_light(mut commands: Commands) {
        // directional 'sun' light
    const HALF_SIZE: f32 = 10.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..Default::default()
            },
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut positions: Query<&mut Transform, With<Player>>,
    // the player mesh
    mut player_mesh: Query<&mut Mesh, With<Player>>,
) {
    for mut transform in positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 0.5;
            player_mesh.
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 0.5;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.z -= 0.5;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.z += 0.5;
        }
    }
}
