use std::time::Duration;

use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
    window::PrimaryWindow,
};

#[derive(Debug, Resource, Default)]
pub struct MouseAction {
    pub acts: usize,
    pub pos: Option<Vec2>,
    pub scroll: Vec2,
    pub left: ButtonAction,
    pub right: ButtonAction,
    pub wheel: ButtonAction,
}

#[derive(Debug, Default)]
pub enum ButtonAction {
    #[default]
    None,
    Pressed(OptCheck),
    Clicked(Vec2),
    Dragged(Vec2),
}

#[derive(Debug)]
pub struct OptCheck {
    pub dur: Duration,
    pub act_pos: Vec2,
    pub delta: Vec2,
}

impl MouseAction {
    #[inline]
    pub fn is_none(&self) -> bool {
        self.acts == 0
    }

    #[inline]
    pub fn is_scroll(&self) -> bool {
        self.scroll != Vec2::ZERO
    }
}

impl ButtonAction {
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
        matches!(self, Self::Clicked(_))
    }

    #[inline]
    pub fn is_dragged(&self) -> bool {
        matches!(self, Self::Dragged(_))
    }
}

impl OptCheck {
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
    mut state: ResMut<MouseAction>,
    pos: Single<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
) {
    state.pos = pos.cursor_position();

    if scroll.delta != Vec2::ZERO && state.scroll == Vec2::ZERO {
        state.acts += 1;
    } else if scroll.delta == Vec2::ZERO && state.scroll != Vec2::ZERO {
        state.acts -= 1;
    }

    state.scroll = scroll.delta;

    macro_rules! check_button {
        ($button:ident, $name:ident) => {
            if mouse.just_pressed(MouseButton::$button) {
                state.$name = ButtonAction::Pressed(OptCheck::new(state.pos.unwrap()));
                state.acts += 1;
            } else if mouse.pressed(MouseButton::$button) {
                if let ButtonAction::Dragged(delta) = &mut state.$name {
                    *delta = motion.delta;
                } else {
                    let (dur, delta) = {
                        let ButtonAction::Pressed(OptCheck { dur, delta, .. }) = &mut state.$name
                        else {
                            return;
                        };
                        *dur += time.delta();
                        *delta += motion.delta;
                        (dur, delta)
                    };

                    if dur.as_secs_f32() > 0.2 || delta.length_squared() > 4.0 {
                        state.$name = ButtonAction::Dragged(*delta);
                    }
                }
            } else if mouse.just_released(MouseButton::$button) {
                if let ButtonAction::Pressed(OptCheck { act_pos, .. }) = state.$name {
                    state.$name = ButtonAction::Clicked(act_pos);
                } else if let ButtonAction::Dragged(_) = state.$name {
                    state.$name = ButtonAction::None;
                    state.acts -= 1;
                }
            } else if state.$name.is_clicked() {
                state.$name = ButtonAction::None;
                state.acts -= 1;
            }
        };
    }

    check_button!(Left, left);
    check_button!(Right, right);
    check_button!(Middle, wheel);
}
