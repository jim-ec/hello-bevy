mod orbit;

use std::{f32::consts::TAU, process::Termination};

use bevy::{
    core_pipeline::experimental::taa::TemporalAntiAliasing,
    pbr::{CascadeShadowConfig, CascadeShadowConfigBuilder, ScreenSpaceAmbientOcclusion},
    prelude::*,
    render::mesh::PlaneMeshBuilder,
};
use orbit::OrbitControls;

fn main() -> impl Termination {
    App::new()
        .insert_resource(AmbientLight {
            brightness: 1000.,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(orbit::OrbitPlugin)
        .add_systems(Startup, startup)
        .add_systems(Update, update)
        .run()
}

#[derive(Debug, Component)]
struct Marker(f32);

fn startup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Spawn a cube
    commands.spawn((
        Marker(0.0),
        Mesh3d(meshes.add(Mesh::from(Cuboid {
            half_size: Vec3::splat(0.5),
        }))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.5, 0.5, 0.5),
            ..Default::default()
        })),
        Transform::from_xyz(1.0, 0.5, 0.0),
    ));
    commands.spawn((
        Marker(1.5),
        Mesh3d(meshes.add(Mesh::from(Cuboid {
            half_size: Vec3::splat(0.5),
        }))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.5, 0.5, 0.5),
            ..Default::default()
        })),
        Transform::from_xyz(0.0, 0.5, 1.0),
    ));

    // Spawn a plane
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(PlaneMeshBuilder {
            plane: Plane3d {
                normal: Dir3::Y,
                half_size: Vec2::splat(4.0),
            },
            subdivisions: 0,
        }))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.5, 0.5, 0.5),
            ..Default::default()
        })),
    ));

    // Spawn camera
    commands.spawn((
        OrbitControls {
            target: Vec3::ZERO,
            yaw: f32::to_radians(135.0),
            pitch: f32::to_radians(-45.0),
            radius: 5.0,
        },
        Visibility::default(),
        Camera3d::default(),
        ScreenSpaceAmbientOcclusion::default(),
        TemporalAntiAliasing::default(),
        Msaa::Off,
        children![
            DirectionalLight {
                shadows_enabled: true,
                illuminance: 1000.0,
                ..Default::default()
            },
            CascadeShadowConfig::from(CascadeShadowConfigBuilder {
                maximum_distance: 100.0,
                first_cascade_far_bound: 10.0,
                ..Default::default()
            }),
            Transform::from_rotation(Quat::from_rotation_y(-TAU / 8.0)),
        ],
    ));
}

fn update(
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(&MeshMaterial3d<StandardMaterial>, &Marker)>,
) {
    for (material, &Marker(offset)) in query.iter() {
        materials.get_mut(material).unwrap().base_color = Color::srgb(
            ((offset + time.elapsed_secs()).sin() * 0.5 + 0.5) as f32,
            ((offset + time.elapsed_secs() + TAU / 3.0).sin() * 0.5 + 0.5) as f32,
            ((offset + time.elapsed_secs() + 2.0 * TAU / 3.0).sin() * 0.5 + 0.5) as f32,
        );
    }
}
