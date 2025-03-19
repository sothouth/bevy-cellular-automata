use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, prelude::*, window::PresentMode};

mod camera;
mod input;
mod world_grid;

mod cell;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::srgb(0.5, 0.5, 0.5)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Cellular Automata".to_string(),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(input::InputPlugin)
        .add_plugins(cell::CellPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(world_grid::GridPlugin);

    app.run();
}
