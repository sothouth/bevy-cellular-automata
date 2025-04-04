use bevy::prelude::*;

use crate::input;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(
                Update,
                drag_camera.run_if(|mouse: Res<input::MouseAct>| mouse.right.is_dragged()),
            )
            .add_systems(
                Update,
                zoom.run_if(|mouse: Res<input::MouseAct>| mouse.is_scroll()),
            );
    }
}

fn spawn_camera(mut cmd: Commands) {
    cmd.spawn(Camera2d);
}

pub fn drag_camera(
    camera: Single<(&mut Transform, &Projection), With<Camera>>,
    mouse: Res<input::MouseAct>,
    // delta: Res<AccumulatedMouseMotion>,
) {
    let (mut camera, projection) = camera.into_inner();
    let Projection::Orthographic(projection) = projection else {
        return;
    };
    let input::CBAct::Dragged(delta) = mouse.right else {
        return;
    };
    let delta = delta * projection.scale;
    camera.translation.x -= delta.x;
    camera.translation.y += delta.y;
}

fn zoom(
    projection: Single<&mut Projection, With<Camera>>,
    mouse: Res<input::MouseAct>,
    // scroll: Res<AccumulatedMouseScroll>,
) {
    // let orthographic_projection = camera.into_inner();
    let Projection::Orthographic(projection) = &mut *projection.into_inner() else {
        return;
    };
    projection.scale *= 1.0 - mouse.scroll.y * 0.1;
    // camera.scale *= scale;
    // camera.translation *= scale;
}

/// return (min, max, cet)
#[inline]
pub fn correct_camera(
    (camera, transform, projection): (&Camera, &Transform, &Projection),
) -> (Vec2, Vec2, Vec2) {
    let Projection::Orthographic(projection) = projection else {
        return (Vec2::ZERO, Vec2::ZERO, Vec2::ZERO);
    };
    let rect = camera.logical_viewport_rect().unwrap();
    let Rect { mut min, mut max } = rect;
    let scale = projection.scale;
    min *= scale;
    max *= scale;
    let cet = (min + max) * 0.5;
    min -= cet;
    max -= cet;
    let cet = transform.translation.truncate();
    min += cet;
    max += cet;

    (min, max, cet)
}
