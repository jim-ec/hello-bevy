use std::{f32::consts::TAU, process::Termination};

use bevy::{prelude::*, render::mesh::PlaneMeshBuilder};

fn main() -> impl Termination {
    App::new()
        .insert_resource(AmbientLight {
            brightness: 1000.,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(camera::CameraPlugin)
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

mod camera {
    use std::f32::consts::TAU;

    use bevy::{
        core_pipeline::experimental::taa::TemporalAntiAliasing,
        input::{
            gestures::PinchGesture,
            mouse::{MouseScrollUnit, MouseWheel},
        },
        pbr::{CascadeShadowConfig, CascadeShadowConfigBuilder, ScreenSpaceAmbientOcclusion},
        prelude::*,
    };

    pub struct CameraPlugin;

    #[derive(Component)]
    pub struct UserCamera {
        target: Vec3,
        yaw: f32,
        pitch: f32,
        radius: f32,
    }

    impl Plugin for CameraPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(Startup, startup)
                .add_systems(Update, update);
        }
    }

    fn startup(mut commands: Commands) {
        commands
            .spawn((
                UserCamera {
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
            ))
            .with_children(|commands| {
                commands.spawn((
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
                ));
            });
    }

    pub fn update(
        mut query: Query<(&mut UserCamera, &mut Transform)>,
        mut wheel: EventReader<MouseWheel>,
        mut magnify: EventReader<PinchGesture>,
        keys: Res<ButtonInput<KeyCode>>,
    ) {
        let mut magnification = 0.0;
        let mut scroll = Vec2::ZERO;

        for event in magnify.read() {
            magnification += event.0;
        }

        for event in wheel.read() {
            match event.unit {
                MouseScrollUnit::Line => {
                    magnification -= 0.01 * event.y;
                }
                MouseScrollUnit::Pixel => {
                    scroll += Vec2::new(event.x, event.y);
                }
            }
        }

        for (mut camera, mut transform) in query.iter_mut() {
            camera.yaw -= 0.01 * scroll.x;
            camera.pitch -= 0.01 * scroll.y;
            camera.pitch = camera.pitch.clamp(-TAU / 4.0, TAU / 4.0);
            camera.radius *= 1.0 - magnification;

            let yaw = Quat::from_rotation_y(camera.yaw);
            let pitch = Quat::from_rotation_x(camera.pitch);

            camera.target += yaw
                * 0.25
                * Vec3::new(
                    keys.pressed(KeyCode::KeyD) as i8 as f32
                        - keys.pressed(KeyCode::KeyA) as i8 as f32,
                    0.0,
                    keys.pressed(KeyCode::KeyS) as i8 as f32
                        - keys.pressed(KeyCode::KeyW) as i8 as f32,
                )
                .normalize_or_zero();

            let rotation = yaw * pitch;
            let translation = rotation * Vec3::Z * camera.radius + camera.target;

            transform.translation = translation;
            transform.rotation = rotation;
        }
    }
}
