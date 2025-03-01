use bevy::prelude::*;

use crate::camera_control::correct_camera;

#[derive(Component)]
pub struct GridLine;

pub fn draw_arrow(mut cmd: Commands) {
    let mut sprite = Sprite::from_color(Color::srgb(0., 0.9, 0.), Vec2::new(1., 16.));

    cmd.spawn((Transform::from_xyz(0., 8., 0.), sprite.clone()));

    sprite.color = Color::srgb(0.9, 0., 0.);
    sprite.custom_size = Some(Vec2::new(16., 1.));

    cmd.spawn((Transform::from_xyz(8., 0., 0.), sprite));
}

pub fn draw_grid(
    mut cmd: Commands,
    camera: Single<(&Camera, &Transform)>,
    lines: Query<Entity, With<GridLine>>,
) {
    lines
        .into_iter()
        .for_each(|line| cmd.entity(line).despawn());

    let (min, max, cet) = correct_camera(camera.into_inner());

    let mut sprite = Sprite::from_color(Color::WHITE, Vec2::new(1., max.y - min.y));

    for i in ((min.x as isize + 0xf) & !0xf..=max.x.ceil() as isize).step_by(16) {
        cmd.spawn((
            GridLine,
            Transform::from_xyz(i as f32, cet.y, 0.),
            sprite.clone(),
        ));
    }

    sprite.custom_size = Some(Vec2::new(max.x - min.x, 1.));

    for i in ((min.y as isize + 0xf) & !0xf..=max.y.ceil() as isize).step_by(16) {
        cmd.spawn((
            GridLine,
            Transform::from_xyz(cet.x, i as f32, 0.0),
            sprite.clone(),
        ));
    }
}
