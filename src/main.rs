use bevy::prelude::*;

mod custom_app;

mod diagnostic;

mod render;

mod camera;
mod input;
mod world_grid;

mod cell;

fn main() {
    let mut app = App::new();

    app.add_plugins(custom_app::CustomAppPlugin)
        .add_plugins(diagnostic::FrameDiagnosticsPlugin)
        .add_plugins(input::InputPlugin)
        .add_plugins(cell::CellPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(world_grid::GridPlugin);

    app.run();
}
