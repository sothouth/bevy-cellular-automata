use bevy::prelude::*;

const DRAW_LINE: &str = "lines.wgsl";
const DRAW_RECT: &str = "rects.wgsl";

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        // app.add_asset::<MeshMaterial2d<>>()
        //     .add_asset::<RectMaterial>()
        //     .add_system_to_stage(PostUpdate, draw_lines.system())
        //     .add_system_to_stage(PostUpdate, draw_rects.system());
    }
}