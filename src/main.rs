use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(camera::CameraPlugin {})
        .add_systems(Startup, startup)
        .run()
}

fn startup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
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

mod camera {
    use bevy::{
        input::{mouse::MouseWheel, touchpad::TouchpadMagnify},
        prelude::*,
    };

    pub struct CameraPlugin {}

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
                    yaw: f32::to_radians(15.0),
                    pitch: f32::to_radians(-40.0),
                    radius: 5.0,
                },
                Camera3dBundle::default(),
                VisibilityBundle::default(),
            ))
            .with_children(|commands| {
                commands.spawn((DirectionalLightBundle {
                    directional_light: DirectionalLight {
                        shadows_enabled: true,
                        ..Default::default()
                    },
                    ..default()
                },));
            });
    }

    pub fn update(
        mut query: Query<(&mut UserCamera, &mut Transform)>,
        mut wheel: EventReader<MouseWheel>,
        mut magnify: EventReader<TouchpadMagnify>,
    ) {
        let magnify: f32 = magnify.read().map(|m| m.0).sum();

        let wheel: Vec2 = wheel
            .read()
            .map(|&MouseWheel { x, y, .. }| Vec2::new(x, y))
            .sum::<Vec2>();

        for (mut camera, mut transform) in query.iter_mut() {
            camera.yaw -= 0.01 * wheel.x;
            camera.pitch -= 0.01 * wheel.y;
            camera.pitch = camera
                .pitch
                .clamp(-std::f32::consts::PI / 2.0, std::f32::consts::PI / 2.0);
            camera.radius *= 1.0 - magnify;
            // camera.radius = camera.radius.clamp(0.1 * N as f32, 2.0 * N as f32);

            let yaw = Quat::from_rotation_y(camera.yaw);
            let pitch = Quat::from_rotation_x(camera.pitch);
            let rotation = yaw * pitch;
            let translation = rotation * Vec3::Z * camera.radius + camera.target;
            transform.translation = translation;
            transform.rotation = rotation;
        }
    }
}
