use crate::game::board::play_board;
use crate::game::cell_state::{CellValue, FixedCell};
use crate::game::control::control_board;
use crate::game::position::CellPosition;
use crate::GameState;
use bevy::color::palettes::basic::BLACK;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use std::ops::BitOrAssign;
use sudoku::bitset::Set;
use sudoku::board::{CellState, Digit};
use sudoku::strategy::StrategySolver;
use sudoku::Sudoku;

mod board;
mod cell_state;
mod control;
mod position;

pub struct SudokuPlugin;

#[derive(Resource, Debug)]
pub struct SudokuManager {
    pub current_sudoku: Sudoku,
    pub solver: StrategySolver,
}

/// This plugin handles player related stuff like movement
/// Player game is only active during the State `GameState::Playing`
impl Plugin for SudokuPlugin {
    fn build(&self, app: &mut App) {
        control::plugin(app);
        app.add_systems(OnEnter(GameState::Playing), (setup_ui, init_cells).chain())
            .add_systems(
                Update,
                (update_cell, set_keyboard_input).run_if(in_state(GameState::Playing)),
            )
            .add_observer(on_select_cell)
            .add_observer(on_unselect_cell)
            .add_observer(check_solver)
            .add_observer(check_conflict)
            .add_observer(kick_candidates);
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/franklin-normal-600.ttf");
    commands
        .spawn((
            Node {
                // 使用网格布局
                display: Display::Grid,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                // 网格有两列，第一列宽度为内容宽度，第二列宽度为剩余空间
                grid_template_columns: vec![GridTrack::min_content(), GridTrack::flex(1.0)],
                // 网格有两行，第一行高度为内容高度，第二行占据剩余空间
                grid_template_rows: vec![
                    GridTrack::px(60.),
                    // GridTrack::auto(),
                    GridTrack::flex(1.0),
                    GridTrack::px(60.),
                ],
                ..default()
            },
            BackgroundColor(Color::WHITE),
        ))
        .with_children(|builder| {
            // 顶部菜单栏
            builder
                .spawn(Node {
                    display: Display::Grid,
                    // Make this node span two grid columns so that it takes up the entire top tow
                    grid_column: GridPlacement::span(2),
                    padding: UiRect::all(Val::Px(6.0)),
                    ..default()
                })
                .with_children(|builder| {
                    builder.spawn((
                        Text::new("top-level grid"),
                        TextFont {
                            font: font.clone(),
                            ..default()
                        },
                        TextColor::BLACK,
                    ));
                });

            // 格子布局容器
            play_board(&font, builder);

            // 右侧边栏
            control_board(&font, builder);
        });
}

///  选中的格子
#[derive(Component)]
pub struct SelectedCell;

/// 格子背景索引
#[derive(Component)]
pub struct CellGrid;

/// 数字格子
#[derive(Component)]
pub struct DigitCell;

/// 候选格子
#[derive(Component)]
pub struct CandidatesContainer;

/// 候选数字格子索引，从1到9
#[derive(Component, Debug)]
pub struct CandidateCell {
    pub index: u8,
    pub selected: bool,
}

#[derive(Component)]
struct ControlLayout;

fn init_cells(mut commands: Commands, cell_background: Query<(Entity, &CellPosition)>) {
    let sudoku = Sudoku::generate();
    info!("sudoku: {:?}", sudoku);

    let solver = StrategySolver::from_sudoku(sudoku.clone());
    commands.insert_resource(SudokuManager {
        current_sudoku: sudoku,
        solver: solver.clone(),
    });

    'l: for (index, cell_state) in solver.grid_state().into_iter().enumerate() {
        let cell_value = CellValue::new(cell_state);

        for (entity, cell_position) in cell_background.iter() {
            if cell_position.0 == index as u8 {
                match &cell_value.current() {
                    // 如果一开始就是数字，那么这个格子是固定颜色
                    CellState::Digit(_) => {
                        commands
                            .entity(entity)
                            .insert(FixedCell)
                            .insert(cell_value)
                            .insert(BackgroundColor(FIXED_COLOR));
                    }
                    CellState::Candidates(_) => {
                        commands.entity(entity).insert(cell_value);
                    }
                }

                continue 'l;
            }
        }
    }
}

const SELECTED_COLOR: Color = Color::linear_rgb(0.902, 0.773, 0.);
const FIXED_COLOR: Color = Color::linear_rgb(0.914, 0.914, 0.914);

fn on_select_cell(trigger: Trigger<OnInsert, SelectedCell>, mut cell: Query<&mut BackgroundColor>) {
    let entity = trigger.entity();
    if let Ok(mut background) = cell.get_mut(entity) {
        background.0 = SELECTED_COLOR;
    }
}

fn on_unselect_cell(
    trigger: Trigger<OnRemove, SelectedCell>,
    mut cell: Query<(&mut BackgroundColor, Option<&FixedCell>)>,
) {
    let entity = trigger.entity();
    if let Ok((mut background, opt_fixed)) = cell.get_mut(entity) {
        if opt_fixed.is_some() {
            background.0 = FIXED_COLOR;
        } else {
            background.0 = Color::WHITE;
        }
    }
}

fn update_cell(
    mut commands: Commands,
    cell: Query<(&CellValue, &Children, Option<&FixedCell>, &CellPosition), Changed<CellValue>>,
    mut digit_cell: Query<
        (&mut Text, &mut Visibility),
        (With<DigitCell>, Without<CandidatesContainer>),
    >,
    mut candidates_container: Query<
        (&mut Visibility, &Children),
        (With<CandidatesContainer>, Without<DigitCell>),
    >,
    mut candidate_cell: Query<
        (&mut TextColor, &mut Visibility, &mut CandidateCell),
        (Without<DigitCell>, Without<CandidatesContainer>),
    >,
) {
    for (cell_value, children, opt_fixed, cell_position) in cell.iter() {
        for child in children.iter() {
            if let Ok((_text, mut visibility)) = digit_cell.get_mut(*child) {
                *visibility = Visibility::Hidden;
            }
            if let Ok((mut visibility, _children)) = candidates_container.get_mut(*child) {
                *visibility = Visibility::Hidden;
            }
            match cell_value.current() {
                CellState::Digit(digit) => {
                    if let Ok((mut text, mut visibility)) = digit_cell.get_mut(*child) {
                        debug!("cell {} change to digit {}", cell_position, digit.get());
                        text.0 = digit.get().to_string();
                        *visibility = Visibility::Visible;
                        commands.trigger(CheckSolver);
                        commands.trigger(NewValueChecker {
                            digit: digit.get(),
                            position: *cell_position,
                        });
                    }
                }
                CellState::Candidates(candidates) => {
                    if opt_fixed.is_some() {
                        continue;
                    }

                    debug!(
                        "cell {} change to candidates {:?}",
                        cell_position,
                        candidates.into_iter().collect::<Vec<_>>()
                    );

                    if let Ok((mut visibility, children)) = candidates_container.get_mut(*child) {
                        *visibility = Visibility::Visible;

                        for child in children {
                            if let Ok((mut text_color, mut visibility, mut cell)) =
                                candidate_cell.get_mut(*child)
                            {
                                if candidates.contains(Digit::new(cell.index).as_set()) {
                                    cell.selected = true;
                                    *text_color = TextColor(Color::srgb_u8(18, 18, 18));
                                } else {
                                    cell.selected = false;
                                    *text_color = TextColor(Color::srgba_u8(18, 18, 18, 0));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn on_click_cell(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    exist: Query<Entity, With<SelectedCell>>,
) {
    for entity in exist.iter() {
        commands.entity(entity).remove::<SelectedCell>();
    }

    commands.entity(trigger.entity()).insert(SelectedCell);
}

fn set_keyboard_input(
    mut commands: Commands,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut selected_cell: Single<
        (&mut CellValue, Option<&FixedCell>, &CellPosition),
        With<SelectedCell>,
    >,
) {
    let (mut selected_cell, opt_fixed, cell_position) = selected_cell.into_inner();
    if opt_fixed.is_some() {
        return;
    }
    for event in keyboard_input_events.read() {
        if event.state != ButtonState::Pressed {
            continue;
        }

        match event.key_code {
            KeyCode::Digit0 | KeyCode::Numpad0 => {
                selected_cell.set(CellState::Digit(Digit::new(0)));
            }
            KeyCode::Digit1 | KeyCode::Numpad1 => {
                selected_cell.set(CellState::Digit(Digit::new(1)));
            }
            KeyCode::Digit2 | KeyCode::Numpad2 => {
                selected_cell.set(CellState::Digit(Digit::new(2)));
            }
            KeyCode::Digit3 | KeyCode::Numpad3 => {
                selected_cell.set(CellState::Digit(Digit::new(3)));
            }
            KeyCode::Digit4 | KeyCode::Numpad4 => {
                selected_cell.set(CellState::Digit(Digit::new(4)));
            }
            KeyCode::Digit5 | KeyCode::Numpad5 => {
                selected_cell.set(CellState::Digit(Digit::new(5)));
            }
            KeyCode::Digit6 | KeyCode::Numpad6 => {
                selected_cell.set(CellState::Digit(Digit::new(6)));
            }
            KeyCode::Digit7 | KeyCode::Numpad7 => {
                selected_cell.set(CellState::Digit(Digit::new(7)));
            }
            KeyCode::Digit8 | KeyCode::Numpad8 => {
                selected_cell.set(CellState::Digit(Digit::new(8)));
            }
            KeyCode::Digit9 | KeyCode::Numpad9 => {
                selected_cell.set(CellState::Digit(Digit::new(9)));
            }
            KeyCode::Delete => {
                if let CellState::Digit(digit) = selected_cell.current() {
                    commands.trigger(NewValueChecker {
                        digit: digit.get(),
                        position: *cell_position,
                    });
                    selected_cell.rollback();
                }
            }

            _ => {}
        }
    }
}

fn candidate_cell_move(
    trigger: Trigger<Pointer<Over>>,
    mut cell: Query<(Entity, &mut TextColor, &CandidateCell)>,
    parent_query: Query<&Parent>,
    q_select: Query<&SelectedCell>,
) {
    let (entity, mut text_color, candidate_cell) = cell.get_mut(trigger.entity()).unwrap();
    for ancestor in parent_query.iter_ancestors(entity) {
        if q_select.get(ancestor).is_ok() && !candidate_cell.selected {
            *text_color = TextColor(Color::srgba_u8(18, 18, 18, 200))
        }
    }
}

fn candidate_cell_out(
    out: Trigger<Pointer<Out>>,
    mut cell: Query<(Entity, &mut TextColor, &CandidateCell)>,
    parent_query: Query<&Parent>,
    q_select: Query<&SelectedCell>,
) {
    let (entity, mut text_color, candidate_cell) = cell.get_mut(out.entity()).unwrap();
    for ancestor in parent_query.iter_ancestors(entity) {
        if q_select.get(ancestor).is_ok() && !candidate_cell.selected {
            *text_color = TextColor(Color::srgba_u8(18, 18, 18, 0))
        }
    }
}

fn candidate_cell_click(
    click: Trigger<Pointer<Click>>,
    mut cell: Query<&mut CandidateCell>,
    parent_query: Query<&Parent>,
    mut q_select: Query<&mut CellValue, With<SelectedCell>>,
) {
    let mut candidate_cell = cell.get_mut(click.entity()).unwrap();
    for ancestor in parent_query.iter_ancestors(click.entity()) {
        if let Ok(mut cell_value) = q_select.get_mut(ancestor) {
            if let CellState::Candidates(mut candidates) = cell_value.current() {
                if candidate_cell.selected {
                    candidate_cell.selected = false;
                    candidates.remove(Digit::new(candidate_cell.index).as_set());
                } else {
                    candidate_cell.selected = true;
                    candidates.bitor_assign(Digit::new(candidate_cell.index).as_set());
                }

                cell_value.set(CellState::Candidates(candidates));
            }
        }
    }
}

#[derive(Event)]
struct CheckSolver;

fn check_solver(
    _trigger: Trigger<CheckSolver>,
    mut cell_query: Query<(&mut CellValue, &CellPosition)>,
    mut sudoku_manager: ResMut<SudokuManager>,
) {
    let mut list = [CellState::Candidates(Set::NONE); 81];
    for (cell_value, cell_position) in cell_query
        .iter()
        .sort_by::<&CellPosition>(|t1, t2| t1.0.cmp(&t2.0))
    {
        list[cell_position.0 as usize] = cell_value.current().clone();
    }
    sudoku_manager.solver = StrategySolver::from_grid_state(list);

    if sudoku_manager.solver.is_solved() {
        info!("Sudoku solved!");
    }
}

#[derive(Event)]
pub struct NewValueChecker {
    pub digit: u8,
    pub position: CellPosition,
}

fn kick_candidates(
    trigger: Trigger<NewValueChecker>,
    mut q_cell: Query<(&mut CellValue, &CellPosition)>,
) {
    let digit = Digit::new(trigger.event().digit);
    let kicker_position = trigger.event().position.clone();

    for (mut cell_value, cell_position) in q_cell.iter_mut() {
        if kicker_position.row() == cell_position.row()
            || kicker_position.col() == cell_position.col()
            || kicker_position.block() == cell_position.block()
        {
            if let CellState::Candidates(mut candidates) = cell_value.current() {
                candidates.remove(digit.as_set());
                cell_value.set(CellState::Candidates(candidates));
            }
        }
    }
}

fn check_conflict(
    trigger: Trigger<NewValueChecker>,
    mut q_cell: Query<(&CellValue, &CellPosition)>,
) {
    let digit = Digit::new(trigger.event().digit);
    let cell_position = trigger.event().position.clone();

    for (other_cell_value, other_cell_position) in q_cell.iter() {
        if cell_position.row() == other_cell_position.row()
            || cell_position.col() == other_cell_position.col()
            || cell_position.block() == other_cell_position.block()
        {
            if let CellState::Digit(other_digit) = other_cell_value.current() {
                if digit == *other_digit && cell_position != *other_cell_position {
                    info!(
                        "{} {} Conflict detected! {:}",
                        cell_position,
                        digit.get(),
                        other_cell_position
                    );
                }
            }
        }
    }
}
