use bevy::{prelude::*, utils::hashbrown::HashMap};

use crate::input;

pub struct CellPlugin;

impl Plugin for CellPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<Game>()
            .init_resource::<CellControlType>()
            .add_systems(
                PreUpdate,
                handle_space
                    .run_if(|kb: Res<ButtonInput<KeyCode>>| kb.just_pressed(KeyCode::Space)),
            )
            .insert_resource(StepTimer(Timer::from_seconds(STEP, TimerMode::Repeating)))
            .add_systems(
                Update,
                (
                    handle_swap.run_if(|mouse: Res<input::MouseAct>| {
                        mouse.left.is_clicked()
                            || mouse.left.is_dragged()
                            || mouse.left.is_end_drag()
                    }),
                    update,
                )
                    .chain(),
            );
    }
}

const CELL_SIZE: u32 = 16;
const OFFSET: Vec2 = Vec2::new(8., 8.);
const STEP: f32 = 0.5;

const SURVIVE_NUM: usize = 2;
const BIRTH_NUM: usize = 3;
const DIE_NUM: usize = 4;
const NEIGHBORS: [IVec2; 8] = [
    IVec2::new(-1, -1),
    IVec2::new(-1, 0),
    IVec2::new(-1, 1),
    IVec2::new(0, -1),
    IVec2::new(0, 1),
    IVec2::new(1, -1),
    IVec2::new(1, 0),
    IVec2::new(1, 1),
];

#[derive(Debug, Component)]
// #[require(Sprite(|| Sprite::from_color(Color::WHITE, Vec2::new(CELL_SIZE as f32, CELL_SIZE as f32))), Transform(|cell:&Cell| get_pos(&cell.idx)))]
pub struct Cell {
    pub idx: IVec2,
}

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, Copy, States)]
pub enum Game {
    #[default]
    Running,
    Paused,
}

impl Game {
    pub fn is_running(&self) -> bool {
        matches!(self, Self::Running)
    }

    pub fn not(&self) -> Self {
        match self {
            Self::Running => Self::Paused,
            Self::Paused => Self::Running,
        }
    }
}

#[derive(Resource)]
pub struct StepTimer(pub Timer);

#[derive(Debug, Resource, Default)]
pub struct CellControlType(Option<bool>);

pub fn handle_space(cur: Res<State<Game>>, mut nxt: ResMut<NextState<Game>>) {
    nxt.set(cur.not());
}

#[inline]
fn get_pos(idx: &IVec2) -> Transform {
    Transform::from_xyz(
        OFFSET.x + idx.x as f32 * CELL_SIZE as f32,
        OFFSET.y + idx.y as f32 * CELL_SIZE as f32,
        0.,
    )
}

#[inline]
fn get_idx(pos: &Vec2) -> IVec2 {
    IVec2::new(
        ((pos.x - OFFSET.x) / CELL_SIZE as f32).round() as i32,
        ((pos.y - OFFSET.y) / CELL_SIZE as f32).round() as i32,
    )
}

#[inline]
fn new_cell(idx: IVec2) -> (Cell, Transform, Sprite) {
    (
        Cell { idx },
        get_pos(&idx),
        Sprite::from_color(Color::WHITE, Vec2::new(CELL_SIZE as f32, CELL_SIZE as f32)),
    )
}

pub fn update(
    time: Res<Time>,
    mut timer: ResMut<StepTimer>,
    state: Res<State<Game>>,
    cells: Query<(Entity, &Cell)>,
    mut cmd: Commands,
) {
    if !state.is_running() {
        return;
    }
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let mut next = HashMap::with_capacity(cells.iter().len());
    for (_, cell) in cells.iter() {
        NEIGHBORS.iter().for_each(|d| {
            *next.entry(cell.idx + d).or_insert(0) += 1;
        });
    }
    cells.iter().for_each(|(entity, cell)| {
        let count = next.get(&cell.idx).copied().unwrap_or(0);
        // cause double cell
        // if !(count >= SURVIVE_NUM && count < DIE_NUM) {
        if count != SURVIVE_NUM {
            cmd.entity(entity).despawn();
        }
    });
    for (idx, count) in next {
        if count == BIRTH_NUM {
            cmd.spawn(new_cell(idx));
        }
    }
}

pub fn handle_swap(
    mut cmd: Commands,
    mouse: Res<input::MouseAct>,
    cells: Query<(Entity, &Cell)>,
    mut ctl_state: ResMut<CellControlType>,
) {
    if mouse.left.is_end_drag() {
        ctl_state.0 = None;
        return;
    }

    let Some(pos) = mouse.pos else {
        return;
    };

    let idx = get_idx(&pos);

    if let Some(entity) = cells
        .iter()
        .find(|(_, cell)| cell.idx == idx)
        .map(|(entity, _)| entity)
    {
        if mouse.left.is_dragged() {
            if ctl_state.0.is_none() {
                ctl_state.0 = Some(false);
            } else if let Some(true) = ctl_state.0 {
                return;
            }
        }
        cmd.entity(entity).despawn();
    } else {
        if mouse.left.is_dragged() {
            if ctl_state.0.is_none() {
                ctl_state.0 = Some(true);
            } else if let Some(false) = ctl_state.0 {
                return;
            }
        }
        cmd.spawn(new_cell(idx));
    }
}
