use std::f32::consts::TAU;

use bevy::{
    input::{
        gestures::PinchGesture,
        mouse::{AccumulatedMouseMotion, MouseScrollUnit, MouseWheel},
    },
    prelude::*,
};

pub struct OrbitPlugin;

#[derive(Component)]
pub struct OrbitControls {
    pub target: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub radius: f32,
}

impl Plugin for OrbitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update);
    }
}

pub fn update(
    mut query: Query<(&mut OrbitControls, &mut Transform)>,
    mut wheel: EventReader<MouseWheel>,
    mut magnify: EventReader<PinchGesture>,
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
) {
    let mut magnification = 0.0;
    let mut scroll = Vec2::ZERO;

    for event in magnify.read() {
        magnification += event.0;
    }

    for event in wheel.read() {
        match event.unit {
            MouseScrollUnit::Line => {
                magnification += 0.1 * event.y;
            }
            MouseScrollUnit::Pixel => {
                scroll += Vec2::new(event.x, event.y);
            }
        }
    }

    if buttons.pressed(MouseButton::Left) {
        scroll += motion.delta;
    }

    for (mut orbit, mut transform) in query.iter_mut() {
        orbit.yaw -= 0.01 * scroll.x;
        orbit.pitch -= 0.01 * scroll.y;
        orbit.pitch = orbit.pitch.clamp(-TAU / 4.0, TAU / 4.0);
        orbit.radius *= 1.0 - magnification;

        let yaw = Quat::from_rotation_y(orbit.yaw);
        let pitch = Quat::from_rotation_x(orbit.pitch);

        let radius = orbit.radius;
        orbit.target += yaw
            * radius
            * 0.05
            * Vec3::new(
                keys.pressed(KeyCode::KeyD) as i8 as f32 - keys.pressed(KeyCode::KeyA) as i8 as f32,
                0.0,
                keys.pressed(KeyCode::KeyS) as i8 as f32 - keys.pressed(KeyCode::KeyW) as i8 as f32,
            )
            .normalize_or_zero();

        let rotation = yaw * pitch;
        let translation = rotation * Vec3::Z * orbit.radius + orbit.target;

        transform.translation = translation;
        transform.rotation = rotation;
    }
}
