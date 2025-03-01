use bevy::prelude::*;

use crate::control;

pub fn drag_camera(
    mut camera: Single<&mut Transform, With<Camera2d>>,
    delta: Res<control::MouseAction>,
    // delta: Res<AccumulatedMouseMotion>,
) {
    let control::ButtonAction::Dragged(delta) = delta.right else {
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
