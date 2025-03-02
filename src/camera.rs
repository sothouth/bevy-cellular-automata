use bevy::prelude::*;

use crate::input;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera).add_systems(
            Update,
            drag_camera.run_if(|mouse: Res<input::MouseAct>| mouse.right.is_dragged()),
        );
    }
}

fn spawn_camera(mut cmd: Commands) {
    cmd.spawn(Camera2d);
}

pub fn drag_camera(
    mut camera: Single<&mut Transform, With<Camera2d>>,
    mouse: Res<input::MouseAct>,
    // delta: Res<AccumulatedMouseMotion>,
) {
    let input::CBAct::Dragged(delta) = mouse.right else {
        return;
    };
    camera.translation.x -= delta.x;
    camera.translation.y += delta.y;
}

/// return (min, max, cet)
#[inline]
pub fn correct_camera((camera, transform): (&Camera, &Transform)) -> (Vec2, Vec2, Vec2) {
    let rect = camera.logical_viewport_rect().unwrap();
    let Rect { mut min, mut max } = rect;
    let cet = rect.center();
    min -= cet;
    max -= cet;
    let cet = transform.translation.truncate();
    min += cet;
    max += cet;

    (min, max, cet)
}
