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

    #[cfg(feature = "release")]
    app.add_startup_system(setup_camera);

    app.add_startup_system(setup_character);
    app.add_startup_system(setup_environment);
    app.add_startup_system(setup_light);

    app.run();
}

fn setup_character(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule {
            ..Default::default()
        })),
        material: materials.add(StandardMaterial {
            base_color: Color::DARK_GRAY,
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..Default::default()
    });
}

fn setup_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 1000.,
        })),
        material: materials.add(StandardMaterial {
            base_color: Color::DARK_GREEN,
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
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
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(50.0, 50.0, 50.0)),
        point_light: PointLight {
            intensity: 600000.,
            range: 100.,
            ..Default::default()
        },
        ..Default::default()
    });
}
