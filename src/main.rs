use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .run()
}

fn startup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Spawn a camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(3.0, 2.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // Spawn a light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(3.0, 2.0, -4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // Spawn a cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid {
            half_size: Vec3::splat(0.5),
        })),
        material: materials.add(StandardMaterial::from(Color::rgb(1.0, 0.0, 0.0))),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });

    // Spawn a plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Plane3d {
            normal: Direction3d::Y,
        })),
        material: materials.add(StandardMaterial::from(Color::rgb(0.0, 1.0, 0.0))),
        ..Default::default()
    });
}
