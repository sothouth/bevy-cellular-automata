use bevy::prelude::*;

mod camera_control;
mod control;
mod world_grid;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::srgb(0.5, 0.5, 0.5)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Cellular Automata".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(First, control::handle_mouse)
        .add_systems(
            Update,
            (world_grid::draw_grid, world_grid::draw_arrow).chain(),
        )
        .init_resource::<control::MouseAction>()
        .add_systems(
            Update,
            camera_control::drag_camera
                .run_if(|state: Res<control::MouseAction>| state.right.is_dragged()),
        );

    app.run();
}

fn setup(mut cmd: Commands) {
    cmd.spawn(Camera2d);
}
