use bevy::prelude::*;

// pub mod keyboard;
// pub use keyboard::{KBAct, handle_keyboard};

pub mod mouse;
pub use mouse::{CBAct, MouseAct, handle_mouse};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseAct>()
            .add_systems(First, handle_mouse);
        // .add_systems(First, handle_keyboard);
    }
}
