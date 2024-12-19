use crate::color::*;
use crate::game::board::play_board;
use crate::game::cell_state::{CellValue, FixedCell};
use crate::game::control::control_board;
use crate::game::input::keyboard_input;
use crate::game::position::CellPosition;
use crate::GameState;
use bevy::color::palettes::basic::RED;
use bevy::prelude::*;
use bevy::utils::HashSet;
use std::ops::BitOrAssign;
use sudoku::bitset::Set;
use sudoku::board::{CellState, Digit};
use sudoku::strategy::StrategySolver;
use sudoku::Sudoku;

mod board;
mod cell_state;
mod control;
mod input;
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
        board::plugin(app);
        app.init_resource::<AutoCandidateMode>()
            .add_systems(OnEnter(GameState::Playing), (setup_ui, init_cells).chain())
            .add_systems(
                Update,
                (
                    keyboard_input,
                    show_conflict,
                    kick_candidates,

                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_observer(on_select_cell)
            .add_observer(on_unselect_cell)
            .add_observer(on_update_cell)
            .add_observer(on_clean_cell)
            .add_observer(check_conflict)
            .add_observer(remove_conflict)
            .add_observer(check_solver);
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font5 = asset_server.load("fonts/franklin-normal-500.ttf");

    commands
        .spawn((
            Name::new("sudoku-content"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,

                ..default()
            },
            // BackgroundColor(RED.into()),
        ))
        .with_children(|builder| {
            // 顶部 LOGO
            title_bar(&asset_server, &font5, builder);

            builder
                .spawn((
                    Name::new("game-content"),
                    Node {
                        height: Val::Vh(90.0),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BackgroundColor(WHITE_COLOR),
                ))
                .with_children(|builder| {
                    // 工具栏
                    toolbars(&asset_server, &font5, builder);

                    // 游戏容器
                    builder
                        .spawn((
                            Name::new("game-root"),
                            Node {
                                height: Val::Percent(100.0),
                                padding: UiRect::all(Val::Px(13.0)),
                                ..default()
                            },
                        ))
                        .with_children(|builder| {
                            builder
                                .spawn(Node {
                                    display: Display::Flex,
                                    align_items: AlignItems::Stretch,
                                    justify_content: JustifyContent::Center,
                                    margin: UiRect::axes(Val::Auto, Val::Px(20.0)),
                                    ..default()
                                })
                                .with_children(|builder| {
                                    // 格子布局容器
                                    play_board(&asset_server, builder);

                                    // 右侧边栏
                                    control_board(&asset_server, &font5, builder);
                                });
                        });
                });
        });
}

fn toolbars(asset_server: &Res<AssetServer>, font5: &Handle<Font>, builder: &mut ChildBuilder) {
    builder
        .spawn((
            Name::new("tool-bar"),
            Node {
                border: UiRect::vertical(Val::Px(1.0)),
                ..default()
            },
            BorderColor(*EXTRA_LIGHT_GRAY),
            BackgroundColor(WHITE_COLOR),
        ))
        .with_children(|builder| {
            builder
                .spawn((
                    Name::new("toolbar-row"),
                    Node {
                        width: Val::Percent(100.0),
                        max_width: Val::Px(1280.0),
                        margin: UiRect::axes(Val::Auto, Val::Px(12.0)),
                        padding: UiRect::axes(Val::Px(24.0), Val::Px(0.0)),
                        display: Display::Flex,
                        flex_wrap: FlexWrap::NoWrap,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    BorderColor(*BLACK),
                ))
                .with_children(|builder| {
                    // left bar
                    left_bar(&asset_server, &font5, builder);
                    // center bar
                    center_bar(&asset_server, &font5, builder);
                    // right bar
                    right_bar(&asset_server, builder);
                });
        });
}

fn right_bar(asset_server: &Res<AssetServer>, builder: &mut ChildBuilder) {
    builder
        .spawn((
            Name::new("right-bar"),
            Node {
                width: Val::Px(350.0),
                margin: UiRect {
                    left: Val::Auto,
                    ..default()
                },
                display: Display::Flex,
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
        ))
        .with_children(|builder| {
            builder.spawn((
                ImageNode {
                    image: asset_server.load("textures/question.png"),
                    ..default()
                },
                Node {
                    width: Val::Px(20.0),
                    margin: UiRect {
                        left: Val::Px(10.0),
                        right: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                },
            ));

            builder.spawn((
                ImageNode {
                    image: asset_server.load("textures/more.png"),
                    ..default()
                },
                Node {
                    width: Val::Px(20.0),
                    margin: UiRect {
                        left: Val::Px(10.0),
                        right: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                },
            ));

            builder.spawn((
                ImageNode {
                    image: asset_server.load("textures/setting.png"),
                    ..default()
                },
                Node {
                    width: Val::Px(20.0),
                    margin: UiRect {
                        left: Val::Px(10.0),
                        right: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                },
            ));
        });
}

fn center_bar(asset_server: &Res<AssetServer>, font: &Handle<Font>, builder: &mut ChildBuilder) {
    builder
        .spawn((
            Name::new("center-bar"),
            Node {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|builder| {
            builder.spawn((
                Text::new("1:02:34"),
                TextFont {
                    font_size: 16.0,
                    font: font.clone(),
                    ..default()
                },
                TextColor(*DARK_BLACK),
            ));

            builder.spawn((
                ImageNode {
                    image: asset_server.load("textures/pause.png"),
                    ..default()
                },
                Node {
                    margin: UiRect {
                        left: Val::Px(5.0),
                        ..default()
                    },
                    width: Val::Px(11.0),
                    ..default()
                },
            ));
        });
}

fn left_bar(asset_server: &Res<AssetServer>, font: &Handle<Font>, builder: &mut ChildBuilder) {
    builder
        .spawn((
            Name::new("left-tool-bar"),
            Node {
                width: Val::Px(350.0),
                margin: UiRect {
                    right: Val::Auto,
                    ..default()
                },
                display: Display::Flex,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
        ))
        .with_children(|builder| {
            builder
                .spawn((Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },))
                .with_children(|builder| {
                    builder.spawn((
                        ImageNode {
                            image: asset_server.load("textures/back.png"),
                            ..default()
                        },
                        Node {
                            // width: Val::Px(11.0),
                            margin: UiRect {
                                right: Val::Px(4.0),
                                ..default()
                            },
                            height: Val::Px(19.0),
                            ..default()
                        },
                    ));

                    builder.spawn((
                        Text::new("Back"),
                        TextFont {
                            font_size: 16.0,
                            font: font.clone(),
                            ..default()
                        },
                        TextColor(*DARK_BLACK),
                    ));
                });
        });
}

/// 顶部标题栏
fn title_bar(asset_server: &Res<AssetServer>, font: &Handle<Font>, builder: &mut ChildBuilder) {
    builder
        .spawn((
            Name::new("title-bar"),
            Node {
                display: Display::Flex,
                ..default()
            },
            BackgroundColor(WHITE_COLOR),
        ))
        .with_children(|builder| {
            builder
                .spawn((
                    Name::new("title-wrapper"),
                    Node {
                        display: Display::Flex,
                        margin: UiRect::axes(Val::Auto, Val::Px(0.0)),
                        padding: UiRect::all(Val::Px(24.0)),
                        max_width: Val::Px(1280.0),
                        width: Val::Px(1280.0),
                        align_items: AlignItems::Baseline,
                        ..default()
                    },
                    // BackgroundColor(GAME_YELLOW),
                ))
                .with_children(|builder| {
                    builder
                        .spawn((
                            Name::new("game-title"),
                            Node {
                                margin: UiRect {
                                    // top: Val::Px(10.0),
                                    right: Val::Px(16.0),
                                    ..default()
                                },
                                // padding: UiRect::axes(Val::Px(5.), Val::Px(1.)),
                                ..default()
                            },
                            // BackgroundColor(GRAY2),
                        ))
                        .with_children(|p| {
                            p.spawn((
                                Text::new("Sudoku"),
                                TextFont {
                                    font_size: 42.0,
                                    font: asset_server.load("fonts/NYTKarnakCondensed.ttf"),
                                    ..default()
                                },
                                TextColor::BLACK,
                            ));
                        });

                    builder
                        .spawn((
                            Name::new("game-date"),
                            Node {
                                bottom: Val::Px(6.0),
                                // padding: UiRect::axes(Val::Px(5.), Val::Px(1.)),
                                ..default()
                            },
                            // BackgroundColor(GRAY),
                        ))
                        .with_children(|p| {
                            p.spawn((
                                Text::new("December 17, 2024"),
                                TextFont {
                                    font_size: 28.0,
                                    font: font.clone(),
                                    ..default()
                                },
                                TextColor::BLACK,
                            ));
                        });
                });
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

/// 冲突红点
#[derive(Component, Default, Deref, DerefMut)]
pub struct ConflictCount(HashSet<Entity>);

/// 候选格子
#[derive(Component)]
pub struct CandidatesContainer;

/// 候选数字格子索引，从1到9
#[derive(Component, Debug)]
pub struct CandidateCell {
    pub index: u8,
    /// 是否是自动选择的候选数字
    pub auto_candidate_selected: bool,
    /// 是否是手动选择的候选数字
    pub manual_candidate_selected: bool,
}

#[derive(Component)]
struct ControlLayout;

fn init_cells(
    mut commands: Commands,
    cell_background: Query<(Entity, &CellPosition)>,
    auto_mode: Res<AutoCandidateMode>,
) {
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
                match &cell_value.current(**auto_mode) {
                    // 如果一开始就是数字，那么这个格子是固定颜色
                    CellState::Digit(_) => {
                        commands
                            .entity(entity)
                            .insert(FixedCell)
                            .insert(cell_value)
                            .insert(BackgroundColor(*EXTRA_LIGHT_GRAY));
                    }
                    CellState::Candidates(_) => {
                        commands.entity(entity).insert(cell_value);
                    }
                }

                if index == 0 {
                    commands.entity(entity).insert(SelectedCell);
                }

                continue 'l;
            }
        }
    }
}

fn on_select_cell(trigger: Trigger<OnInsert, SelectedCell>, mut cell: Query<&mut BackgroundColor>) {
    let entity = trigger.entity();
    if let Ok(mut background) = cell.get_mut(entity) {
        background.0 = *STRANDS_YELLOW;
    }
}

fn on_unselect_cell(
    trigger: Trigger<OnRemove, SelectedCell>,
    mut cell: Query<(&mut BackgroundColor, Option<&FixedCell>)>,
) {
    let entity = trigger.entity();
    if let Ok((mut background, opt_fixed)) = cell.get_mut(entity) {
        if opt_fixed.is_some() {
            background.0 = *EXTRA_LIGHT_GRAY;
        } else {
            background.0 = WHITE_COLOR;
        }
    }
}

fn on_update_cell(
    trigger: Trigger<UpdateCell>,
    mut q_cell: Query<(&mut CellValue, Option<&FixedCell>)>,
    auto_mode: Res<AutoCandidateMode>,
    mut commands: Commands,
) {
    if let Ok((mut cell_value, opt_fixed)) = q_cell.get_mut(trigger.entity()) {
        if opt_fixed.is_none() {
            cell_value.bitor_assign(trigger.event().0, **auto_mode);

            if let (CellState::Digit(_digit), CellState::Candidates(_)) =
                (cell_value.current(**auto_mode), trigger.event().0)
            {
                commands.trigger_targets(CleanCell, vec![trigger.entity()]);
            }
        }
    }
}

fn on_clean_cell(
    trigger: Trigger<CleanCell>,
    mut q_cell: Query<(&mut CellValue, Option<&FixedCell>)>,
    auto_mode: Res<AutoCandidateMode>,
) {
    if let Ok((mut cell_value, opt_fixed)) = q_cell.get_mut(trigger.entity()) {
        if opt_fixed.is_none() {
            cell_value.rollback(**auto_mode);
        }
    }
}

fn candidate_cell_click(
    click: Trigger<Pointer<Click>>,
    mut cell: Query<&mut CandidateCell>,
    parent_query: Query<&Parent>,
    mut q_select: Query<&mut CellValue, With<SelectedCell>>,
    auto_mode: Res<AutoCandidateMode>,
) {
    let mut candidate_cell = cell.get_mut(click.entity()).unwrap();
    for ancestor in parent_query.iter_ancestors(click.entity()) {
        if let Ok(mut cell_value) = q_select.get_mut(ancestor) {
            cell_value.bitor_assign(
                CellState::Candidates(Digit::new(candidate_cell.index).as_set()),
                **auto_mode,
            );
        }
    }
}

fn check_solver(
    _trigger: Trigger<UpdateCell>,
    mut cell_query: Query<(&mut CellValue, &CellPosition)>,
    mut sudoku_manager: ResMut<SudokuManager>,
    auto_mode: Res<AutoCandidateMode>,
) {
    let mut list = [CellState::Candidates(Set::NONE); 81];
    for (cell_value, cell_position) in cell_query
        .iter()
        .sort_by::<&CellPosition>(|t1, t2| t1.0.cmp(&t2.0))
    {
        list[cell_position.0 as usize] = cell_value.current(**auto_mode).clone();
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

#[derive(Event)]
pub struct UpdateCell(pub CellState);

#[derive(Event)]
pub struct CleanCell;

#[derive(Event)]
pub struct CheckConflict;

fn kick_candidates(
    changed_cell: Query<(&CellValue, &CellPosition), (Changed<CellValue>, With<SelectedCell>)>,
    mut q_cell: Query<(&mut CellValue, &CellPosition), Without<SelectedCell>>,
    auto_mode: Res<AutoCandidateMode>,
) {
    for (cell_state, kicker_position) in changed_cell.iter() {
        if let CellState::Digit(digit) = cell_state.current(**auto_mode) {
            info!("kick_candidates: {:?} {} ", digit, kicker_position);

            for (mut cell_value, cell_position) in q_cell.iter_mut() {
                if kicker_position.row() == cell_position.row()
                    || kicker_position.col() == cell_position.col()
                    || kicker_position.block() == cell_position.block()
                {
                    if let CellState::Candidates(mut candidates) = cell_value.current(**auto_mode) {
                        candidates.remove(digit.as_set());
                        cell_value.set(CellState::Candidates(candidates), **auto_mode);
                    }
                }
            }
        }
    }
}

fn check_conflict(
    trigger: Trigger<UpdateCell>,
    update_cell: Query<&CellPosition>,
    mut q_cell: Query<(Entity, &CellValue, &CellPosition, &Children)>,
    mut q_conflict: Query<&mut ConflictCount>,
    auto_mode: Res<AutoCandidateMode>,
) {
    if let CellState::Digit(digit) = trigger.event().0 {
        let check_entity = trigger.entity();
        if let Ok(cell_position) = update_cell.get(check_entity) {
            let mut conflict_list = vec![];
            for (other_entity, other_cell_value, other_cell_position, children) in q_cell.iter() {
                if cell_position.row() == other_cell_position.row()
                    || cell_position.col() == other_cell_position.col()
                    || cell_position.block() == other_cell_position.block()
                {
                    if let CellState::Digit(other_digit) = other_cell_value.current(**auto_mode) {
                        if digit == *other_digit && cell_position != other_cell_position {
                            conflict_list.push(other_entity);
                            for child in children {
                                if let Ok(mut conflict_count) = q_conflict.get_mut(*child) {
                                    conflict_count.insert(check_entity);
                                }
                            }
                        }
                    }
                }
            }

            if !conflict_list.is_empty() {
                for (entity, _other_cell_value, other_cell_position, children) in q_cell.iter() {
                    if other_cell_position == cell_position {
                        for child in children {
                            if let Ok(mut conflict_count) = q_conflict.get_mut(*child) {
                                conflict_count.insert(entity);
                                conflict_count.extend(conflict_list);
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn show_conflict(mut q_conflict: Query<(&mut Visibility, &ConflictCount), Changed<ConflictCount>>) {
    for (mut visibility, conflict_count) in q_conflict.iter_mut() {
        if conflict_count.is_empty() {
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Visible;
        }
    }
}

fn remove_conflict(
    trigger: Trigger<CleanCell>,
    mut q_cell: Query<(&CellValue, &CellPosition, &Children)>,
    mut q_conflict: Query<&mut ConflictCount>,
    auto_mode: Res<AutoCandidateMode>,
) {
    let (cell_value, cell_position, children) = q_cell.get(trigger.entity()).unwrap();
    if let CellState::Digit(digit) = cell_value.current(**auto_mode) {
        for child in children {
            if let Ok(mut conflict_count) = q_conflict.get_mut(*child) {
                conflict_count.clear();
            }
        }

        for (other_cell_value, other_cell_position, children) in q_cell.iter() {
            if cell_position.row() == other_cell_position.row()
                || cell_position.col() == other_cell_position.col()
                || cell_position.block() == other_cell_position.block()
            {
                if let CellState::Digit(other_digit) = other_cell_value.current(**auto_mode) {
                    if digit == other_digit && cell_position != other_cell_position {
                        for child in children {
                            if let Ok(mut conflict_count) = q_conflict.get_mut(*child) {
                                conflict_count.remove(&trigger.entity());
                                println!(
                                    "clean {} conflict count: {}",
                                    other_cell_position,
                                    conflict_count.0.len()
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct AutoCandidateMode(pub bool);
