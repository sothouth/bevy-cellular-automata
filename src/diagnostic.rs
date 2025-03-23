use std::{ops::Add, time::Duration};

use bevy::{ diagnostic::FrameCount, prelude::*};

pub struct FrameDiagnosticsPlugin;

impl Plugin for FrameDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DiagnosticInfo>()
            .add_systems(Update, update_frame_diagnostics);
    }
}

#[derive(Resource)]
pub struct DiagnosticInfo {
    pub pre_frame_num: u32,
    pub max_frame_time: Duration,
    pub timer: Timer,
}

impl Default for DiagnosticInfo {
    fn default() -> Self {
        Self {
            pre_frame_num: 0,
            max_frame_time: Duration::ZERO,
            timer: Timer::from_seconds(1., TimerMode::Repeating),
        }
    }
}

fn update_frame_diagnostics(
    mut info: ResMut<DiagnosticInfo>,
    time: Res<Time<Real>>,
    frame_num: Res<FrameCount>,
) {
    info.timer.tick(time.delta());
    info.max_frame_time = info.max_frame_time.max(time.delta());

    if !info.timer.just_finished() {
        return;
    }

    let average_frame_time = info.timer.elapsed().add(info.timer.duration())
        / frame_num.0.wrapping_sub(info.pre_frame_num);
    let fps = 1.0 / average_frame_time.as_secs_f64();

    info!("fps: {fps}");
    info!("frame time: {average_frame_time:?}");
    info!("max frame time: {:?}", info.max_frame_time);

    info.pre_frame_num = frame_num.0;
    info.max_frame_time = Duration::ZERO;
    info.timer.reset();
}
