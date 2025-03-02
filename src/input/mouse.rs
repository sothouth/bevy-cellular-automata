use std::time::Duration;

use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
    window::PrimaryWindow,
};

#[derive(Debug, Resource, Default)]
pub struct MouseAct {
    pub acts: usize,
    pub pos: Option<Vec2>,
    pub scroll: Vec2,
    pub left: CBAct,
    pub right: CBAct,
    pub wheel: CBAct,
}

/// Complex Button Action
#[derive(Debug, Default)]
pub enum CBAct {
    #[default]
    None,
    Pressed(OptState),
    Clicked,
    Dragged(Vec2),
    EndDrag,
}

#[derive(Debug)]
pub struct OptState {
    pub dur: Duration,
    pub act_pos: Vec2,
    pub delta: Vec2,
}

impl MouseAct {
    #[inline]
    pub fn is_none(&self) -> bool {
        self.acts == 0
    }

    #[inline]
    pub fn is_scroll(&self) -> bool {
        self.scroll != Vec2::ZERO
    }
}

impl CBAct {
    #[inline]
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    #[inline]
    pub fn is_pressed(&self) -> bool {
        matches!(self, Self::Pressed(_))
    }

    #[inline]
    pub fn is_clicked(&self) -> bool {
        matches!(self, Self::Clicked)
    }

    #[inline]
    pub fn is_dragged(&self) -> bool {
        matches!(self, Self::Dragged(_))
    }

    #[inline]
    pub fn is_end_drag(&self) -> bool {
        matches!(self, Self::EndDrag)
    }
}

impl OptState {
    #[inline]
    pub fn new(pos: Vec2) -> Self {
        Self {
            dur: Duration::ZERO,
            act_pos: pos,
            delta: Vec2::ZERO,
        }
    }
}

pub fn handle_mouse(
    time: Res<Time>,
    mut state: ResMut<MouseAct>,
    pos: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &Transform)>,
    mouse: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
) {
    state.pos = pos.cursor_position().map(|pos| {
        let y_reflected_pos = Vec2::new(pos.x, -pos.y);
        let left_top = {
            let (camera, transform) = camera.into_inner();
            let (half_size, center_pos) = (
                camera.logical_viewport_size().unwrap() / 2.,
                transform.translation.truncate(),
            );
            Vec2::new(center_pos.x - half_size.x, center_pos.y + half_size.y)
        };
        y_reflected_pos + left_top
    });

    if scroll.delta != Vec2::ZERO && state.scroll == Vec2::ZERO {
        state.acts += 1;
    } else if scroll.delta == Vec2::ZERO && state.scroll != Vec2::ZERO {
        state.acts -= 1;
    }

    state.scroll = scroll.delta;

    macro_rules! check_button {
        ($button:ident, $name:ident) => {
            if mouse.just_pressed(MouseButton::$button) {
                state.$name = CBAct::Pressed(OptState::new(state.pos.unwrap()));
                state.acts += 1;
            } else if mouse.pressed(MouseButton::$button) {
                if let CBAct::Dragged(delta) = &mut state.$name {
                    *delta = motion.delta;
                } else {
                    let (dur, delta) = {
                        let CBAct::Pressed(OptState { dur, delta, .. }) = &mut state.$name else {
                            return;
                        };
                        *dur += time.delta();
                        *delta += motion.delta;
                        (dur, delta)
                    };

                    if dur.as_secs_f32() > 0.2 || delta.length_squared() > 4.0 {
                        state.$name = CBAct::Dragged(*delta);
                    }
                }
            } else if mouse.just_released(MouseButton::$button) {
                if let CBAct::Pressed(OptState { .. }) = state.$name {
                    state.$name = CBAct::Clicked;
                } else if let CBAct::Dragged(_) = state.$name {
                    state.$name = CBAct::EndDrag;
                }
            } else if state.$name.is_clicked() || state.$name.is_end_drag() {
                state.$name = CBAct::None;
                state.acts -= 1;
            }
        };
    }

    check_button!(Left, left);
    check_button!(Right, right);
    check_button!(Middle, wheel);
}
