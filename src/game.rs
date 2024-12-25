use crate::game::dialog::{dialog_container, ShowHint};
use crate::game::dialog::{DialogContainer, PauseGame};
use crate::share::title_bar;
use crate::{
    color::*,
    game::{
        board::{play_board, PreviewCandidate},
        cell_state::CandidatesValue,
        cell_state::{
            AutoCandidates, CellMode, CellValueBundle, DigitValueCell, FixedCell, ManualCandidates,
        },
        control_tab::control_board,
        input::{keyboard_input, keyboard_move_cell},
        position::CellPosition,
    },
    loading::{FontAssets, TextureAssets},
    GameState,
};
use bevy::color::palettes::basic::RED;
use bevy::color::palettes::css::LIGHT_YELLOW;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy::utils::HashSet;
use sudoku::bitset::Set;
use sudoku::board::{CellState, Digit};
use sudoku::strategy::StrategySolver;
use sudoku::Sudoku;

mod board;
mod cell_state;
mod control_tab;
mod dialog;
mod input;
mod position;

pub struct SudokuPlugin;

/// This plugin handles player related stuff like movement
/// Player game is only active during the State `GameState::Playing`
impl Plugin for SudokuPlugin {
    fn build(&self, app: &mut App) {
        control_tab::plugin(app);
        board::plugin(app);
        dialog::plugin(app);
        app.init_resource::<AutoCandidateMode>()
            .add_event::<MoveSelectCell>()
            .add_event::<NewDigit>()
            .add_event::<NewCandidate>()
            .add_event::<RemoveDigit>()
            .add_event::<CleanCell>()
            .add_systems(OnEnter(GameState::Playing), (setup_ui, init_cells).chain())
            .add_systems(OnExit(GameState::Playing), cleanup_game)
            .add_systems(
                Update,
                (
                    keyboard_input,
                    update_game_time,
                    keyboard_move_cell,
                    show_conflict,
                    kick_candidates,
                    on_new_digit,
                    on_new_candidate,
                    check_solver,
                    on_clean_cell,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_observer(check_conflict)
            .add_observer(on_select_cell)
            .add_observer(remove_conflict)
            .add_observer(on_unselect_cell)
            .add_observer(on_show_more);
    }
}

#[derive(Resource, Debug)]
pub struct SudokuManager {
    pub current_sudoku: Sudoku,
    pub solver: StrategySolver,
}

#[derive(Component)]
struct Game;

fn setup_ui(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    texture_assets: Res<TextureAssets>,
) {
    commands.spawn((Game, Camera2d));
    commands.insert_resource(GameTimer(Stopwatch::new()));
    commands
        .spawn((
            Game,
            Name::new("sudoku-content"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                // margin: UiRect {
                //     top: Val::Px(24.0),
                //     ..default()
                // },
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,

                ..default()
            },
            // BackgroundColor(RED.into()),
        ))
        .with_children(|builder| {
            // 顶部 LOGO
            title_bar(&font_assets, builder);

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
                    toolbars(&font_assets, &texture_assets, builder);

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
                            dialog_container(&font_assets, builder);

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
                                    play_board(&font_assets, &texture_assets, builder);

                                    // 右侧边栏
                                    control_board(&font_assets, &texture_assets, builder);
                                });
                        });
                });
        });
}

fn toolbars(
    font_assets: &Res<FontAssets>,
    texture_assets: &Res<TextureAssets>,
    builder: &mut ChildBuilder,
) {
    builder
        .spawn((
            Name::new("tool-bar"),
            Node {
                border: UiRect::vertical(Val::Px(1.0)),
                ..default()
            },
            BorderColor(*EXTRA_LIGHT_GRAY),
        ))
        .with_children(|builder| {
            builder
                .spawn((
                    Name::new("toolbar-row"),
                    Node {
                        width: Val::Percent(100.0),
                        max_width: Val::Px(1280.0),
                        height: Val::Px(55.0),
                        margin: UiRect::axes(Val::Auto, Val::Px(0.0)),
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
                    left_bar(&font_assets, &texture_assets, builder);
                    // center bar
                    center_bar(&font_assets, &texture_assets, builder);
                    // right bar
                    right_bar(&font_assets, &texture_assets, builder);
                });
        });
}

fn right_bar(
    font_assets: &Res<FontAssets>,
    texture_assets: &Res<TextureAssets>,
    builder: &mut ChildBuilder,
) {
    builder
        .spawn((
            Name::new("right-bar"),
            Node {
                width: Val::Px(350.0),

                margin: UiRect {
                    left: Val::Auto,
                    top: Val::Px(14.0),
                    bottom: Val::Px(14.0),
                    ..default()
                },
                padding: UiRect::all(Val::Px(4.0)),
                display: Display::Flex,
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            // BackgroundColor(Color::linear_rgba(0.1, 0.95, 0.95, 0.5)),
        ))
        .with_children(|builder| {
            builder
                .spawn((
                    ImageNode {
                        image: texture_assets.question.clone(),
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
                ))
                .observe(
                    |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                        commands.trigger(ShowHint(true));
                    },
                );

            builder
                .spawn((
                    Name::new("how to"),
                    ImageNode {
                        image: texture_assets.more.clone(),
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
                ))
                .with_children(|builder| {
                    spawn_show_more(&font_assets, builder);
                })
                .observe(
                    |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                        println!("show more");
                        commands.trigger(ShowMore(true));
                    },
                );

            builder.spawn((
                ImageNode {
                    image: texture_assets.setting.clone(),
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

fn center_bar(
    font_assets: &Res<FontAssets>,
    texture_assets: &Res<TextureAssets>,
    builder: &mut ChildBuilder,
) {
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
                Text::new("Easy"),
                TextFont {
                    font_size: 18.0,
                    font: font_assets.franklin_500.clone(),
                    ..default()
                },
                TextColor::BLACK,
                Node {
                    margin: UiRect::horizontal(Val::Px(16.0)),
                    ..default()
                },
            ));
            builder.spawn((
                Text::new("1:02:34"),
                TextFont {
                    font_size: 16.0,
                    font: font_assets.franklin_500.clone(),
                    ..default()
                },
                TextColor(*DARK_BLACK),
                TimerText,
            ));

            builder
                .spawn((
                    ImageNode {
                        image: texture_assets.pause.clone(),
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
                ))
                .observe(
                    |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                        commands.trigger(PauseGame(true));
                    },
                );
        });
}

fn left_bar(
    font_assets: &Res<FontAssets>,
    texture_assets: &Res<TextureAssets>,
    builder: &mut ChildBuilder,
) {
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
                            image: texture_assets.back.clone(),
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
                            font: font_assets.franklin_500.clone(),
                            ..default()
                        },
                        TextColor(*DARK_BLACK),
                    ));
                })
                .observe(
                    |_trigger: Trigger<Pointer<Click>>,
                     mut next_state: ResMut<NextState<GameState>>| {
                        next_state.set(GameState::Menu);
                    },
                );
        });
}

///  选中的格子
#[derive(Component)]
pub struct SelectedCell;

#[derive(Event)]
pub enum MoveSelectCell {
    Up,
    Down,
    Left,
    Right,
}

/// 数字格子容器
#[derive(Component)]
pub struct DigitCellContainer;

/// 冲突红点
#[derive(Component, Default, Deref, DerefMut)]
pub struct ConflictCount(HashSet<Entity>);

/// 手动候选格子容器
#[derive(Component)]
pub struct ManualCandidatesContainer;

/// 自动候选格子容器
#[derive(Component)]
pub struct AutoCandidatesContainer;

fn init_cells(mut commands: Commands, cell_background: Query<(Entity, &CellPosition)>) {
    let sudoku = Sudoku::generate();
    info!("sudoku: {:?}", sudoku);

    let solver = StrategySolver::from_sudoku(sudoku.clone());
    commands.insert_resource(SudokuManager {
        current_sudoku: sudoku,
        solver: solver.clone(),
    });

    'l: for (index, cell_state) in solver.grid_state().into_iter().enumerate() {
        let bundle = CellValueBundle::from_cell_state(cell_state, false);

        for (entity, cell_position) in cell_background.iter() {
            if cell_position.0 == index as u8 {
                // 如果一开始就是数字，那么这个格子是固定颜色
                if bundle.cell_mode == CellMode::Digit {
                    commands
                        .entity(entity)
                        .insert(bundle)
                        .insert(FixedCell)
                        .insert(BackgroundColor(*EXTRA_LIGHT_GRAY));
                } else {
                    commands.entity(entity).insert(bundle);
                }

                // 如果是第一个格子，那么选中
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

fn on_new_digit(
    mut ev: EventReader<NewDigit>,
    mut q_cell: Query<
        (&mut DigitValueCell, &mut CellMode),
        (With<SelectedCell>, Without<FixedCell>),
    >,
    mut commands: Commands,
) {
    for new_digit in ev.read() {
        for (mut cell_value, mut cell_mode) in q_cell.iter_mut() {
            *cell_mode = CellMode::Digit;
            let new_digit = new_digit.0;

            if let Some(old_digit) = cell_value.0 {
                if old_digit != new_digit {
                    commands.trigger(RemoveDigit(old_digit));
                }
            }

            cell_value.0 = Some(new_digit);
            commands.trigger(CheckDigitConflict)
        }
    }
}

fn on_new_candidate(
    mut trigger: EventReader<NewCandidate>,
    mut q_cell: Query<
        (
            &mut DigitValueCell,
            &mut ManualCandidates,
            &mut AutoCandidates,
            &mut CellMode,
        ),
        (With<SelectedCell>, Without<FixedCell>),
    >,
    auto_mode: Res<AutoCandidateMode>,
    mut commands: Commands,
) {
    for new_candidate in trigger.read() {
        let new_candidate = new_candidate.0;

        for (mut digit_value, mut manual_candidates, mut auto_candidates, mut cell_mode) in
            q_cell.iter_mut()
        {
            debug!("new candidate: {:?}", new_candidate);
            match cell_mode.as_ref() {
                CellMode::Digit => {
                    if let Some(digit) = digit_value.0 {
                        commands.trigger(RemoveDigit(digit));
                    }
                    digit_value.0 = None;
                    if **auto_mode {
                        *cell_mode = CellMode::AutoCandidates;
                        auto_candidates.insert(new_candidate);
                    } else {
                        *cell_mode = CellMode::ManualCandidates;
                        manual_candidates.insert(new_candidate);
                    }
                }
                CellMode::AutoCandidates => {
                    *cell_mode = CellMode::AutoCandidates;
                    auto_candidates.insert(new_candidate);
                }
                CellMode::ManualCandidates => {
                    *cell_mode = CellMode::ManualCandidates;
                    manual_candidates.insert(new_candidate);
                }
            }
        }
    }
}

fn on_clean_cell(
    mut trigger: EventReader<CleanCell>,
    mut q_cell: Query<
        (
            Entity,
            &mut DigitValueCell,
            &mut ManualCandidates,
            &mut CellMode,
        ),
        (With<SelectedCell>, Without<FixedCell>),
    >,
    auto_mode: Res<AutoCandidateMode>,
    children: Query<&Children>,
    q_preview: Query<&PreviewCandidate>,
    mut commands: Commands,
) {
    for _ in trigger.read() {
        for (entity, mut digit_value, mut manual_candidates, mut cell_mode) in q_cell.iter_mut() {
            match *cell_mode {
                CellMode::Digit => {
                    if let Some(digit) = digit_value.0 {
                        commands.trigger(RemoveDigit(digit));
                    }
                    digit_value.0 = None;
                    if **auto_mode {
                        *cell_mode = CellMode::AutoCandidates;
                    } else {
                        *cell_mode = CellMode::ManualCandidates;
                    }
                }
                CellMode::AutoCandidates => {}
                CellMode::ManualCandidates => manual_candidates.0 = Set::NONE,
            }

            for child in children.iter_descendants(entity) {
                if let Ok(_preview) = q_preview.get(child) {
                    commands.entity(child).remove::<PreviewCandidate>();
                }
            }
        }
    }
}

fn check_solver(
    mut _trigger: EventReader<NewDigit>,
    cell_query: Query<(&DigitValueCell, &CellPosition)>,
    mut sudoku_manager: ResMut<SudokuManager>,
) {
    for _ in _trigger.read() {
        let mut list = [CellState::Candidates(Set::NONE); 81];
        for (cell_value, cell_position) in cell_query
            .iter()
            .sort_by::<&CellPosition>(|t1, t2| t1.0.cmp(&t2.0))
        {
            if let Some(digit) = cell_value.0 {
                list[cell_position.0 as usize] = CellState::Digit(digit);
            }
        }

        sudoku_manager.solver = StrategySolver::from_grid_state(list);

        if sudoku_manager.solver.is_solved() {
            info!("Sudoku solved!");
        }
    }
}

#[derive(Event)]
pub struct CleanCell;

#[derive(Event)]
pub struct NewCandidate(pub Digit);

impl NewCandidate {
    pub fn new(digit: u8) -> NewCandidate {
        NewCandidate(Digit::new(digit))
    }
}

#[derive(Event)]
pub struct NewDigit(pub Digit);

impl NewDigit {
    pub fn new(digit: u8) -> NewDigit {
        NewDigit(Digit::new(digit))
    }
}

#[derive(Event)]
pub struct CheckDigitConflict;

#[derive(Event)]
pub struct RemoveDigit(pub Digit);

fn kick_candidates(
    changed_cell: Query<
        (&DigitValueCell, &CellPosition),
        (Changed<DigitValueCell>, With<SelectedCell>),
    >,
    mut q_manual: Query<(&mut ManualCandidates, &CellPosition), Without<SelectedCell>>,
    mut q_auto: Query<(&mut AutoCandidates, &CellPosition), Without<SelectedCell>>,
    auto_mode: Res<AutoCandidateMode>,
) {
    for (cell_state, kicker_position) in changed_cell.iter() {
        if let Some(digit) = cell_state.0 {
            debug!("kick_candidates: {:?} {} ", digit, kicker_position);
            if **auto_mode {
                for (mut auto_candidates, cell_position) in q_auto.iter_mut() {
                    if kicker_position.in_range(cell_position) {
                        auto_candidates.0.remove(digit.as_set());
                    }
                }
            } else {
                for (mut manual_candidates, cell_position) in q_manual.iter_mut() {
                    if kicker_position.in_range(cell_position) {
                        manual_candidates.0.remove(digit.as_set());
                    }
                }
            }
        }
    }
}

fn check_conflict(
    _check_digit: Trigger<CheckDigitConflict>,
    update_cell: Query<
        (Entity, &DigitValueCell, &CellPosition),
        (With<SelectedCell>, Without<FixedCell>),
    >,
    q_cell: Query<(Entity, &DigitValueCell, &CellPosition, &Children)>,
    mut q_conflict: Query<&mut ConflictCount>,
) {
    if let Ok((check_entity, digit_cell, cell_position)) = update_cell.get_single() {
        if let Some(check_digit) = digit_cell.0 {
            debug!("check conflict: {:?}", check_digit);
            let mut conflict_list = vec![];
            for (other_entity, other_cell_value, other_cell_position, children) in q_cell.iter() {
                if cell_position.in_range(other_cell_position) {
                    if let Some(other_digit) = other_cell_value.0 {
                        if check_digit == other_digit && cell_position != other_cell_position {
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
                if let Ok((entity, _other_cell_value, _other_cell_position, children)) =
                    q_cell.get(check_entity)
                {
                    for child in children {
                        if let Ok(mut conflict_count) = q_conflict.get_mut(*child) {
                            conflict_count.insert(entity);
                            conflict_count.extend(conflict_list);
                            return;
                        }
                    }
                }
            }
        };
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
    remove_digit: Trigger<RemoveDigit>,
    q_cell: Query<(Entity, &CellPosition, &Children), With<SelectedCell>>,
    other_cell: Query<(&DigitValueCell, &CellPosition, &Children), Without<SelectedCell>>,
    mut q_conflict: Query<&mut ConflictCount>,
) {
    let remove_digit = remove_digit.0;
    for (entity, cell_position, children) in q_cell.iter() {
        for child in children {
            if let Ok(mut conflict_count) = q_conflict.get_mut(*child) {
                conflict_count.clear();
            }
        }

        for (other_cell_value, other_cell_position, children) in other_cell.iter() {
            if cell_position.in_range(other_cell_position) {
                if let Some(other_digit) = other_cell_value.0 {
                    if remove_digit == other_digit && cell_position != other_cell_position {
                        for child in children {
                            if let Ok(mut conflict_count) = q_conflict.get_mut(*child) {
                                conflict_count.remove(&entity);
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

fn cleanup_game(mut commands: Commands, menu: Query<Entity, With<Game>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Resource, Default, Deref, DerefMut, Debug)]
pub struct GameTimer(pub Stopwatch);

impl core::fmt::Display for GameTimer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let elapsed = self.elapsed();
        let seconds = elapsed.as_secs();
        let minutes = seconds / 60;
        let hours = minutes / 60;
        write!(f, "{:02}:{:02}:{:02}", hours, minutes % 60, seconds % 60)
    }
}

#[derive(Component)]
struct TimerText;

fn update_game_time(
    mut game_timer: ResMut<GameTimer>,
    time: Res<Time>,
    mut text: Single<&mut Text, With<TimerText>>,
) {
    game_timer.tick(time.delta());
    text.0 = game_timer.to_string();
}

#[derive(Event)]
pub struct ShowMore(pub bool);

fn spawn_show_more(font_assets: &Res<FontAssets>, builder: &mut ChildBuilder) {
    builder
        .spawn((
            ShowMoreContainer,
            Visibility::Hidden,
            Name::new("show-more-container"),
            Node {
                top: Val::Px(30.0),
                position_type: PositionType::Absolute,
                width: Val::Px(155.0),
                border: UiRect::all(Val::Px(1.0)),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BorderColor(Color::Srgba(Srgba::hex("c4c4c4").unwrap())),
            BackgroundColor(WHITE_COLOR),
            GlobalZIndex(99),
        ))
        .with_children(|builder| {
            more_item(
                font_assets,
                builder,
                "Hint",
                |_: Trigger<Pointer<Click>>, mut commands| {
                    commands.trigger(ShowMore(false));
                },
            );
            more_item(
                font_assets,
                builder,
                "Check Cell",
                |_: Trigger<Pointer<Click>>, mut commands| {
                    commands.trigger(ShowMore(false));
                },
            );
            more_item(
                font_assets,
                builder,
                "Check Puzzle",
                |_: Trigger<Pointer<Click>>, mut commands| {
                    commands.trigger(ShowMore(false));
                },
            );
            more_item(
                font_assets,
                builder,
                "Reveal Cell",
                |_: Trigger<Pointer<Click>>, mut commands| {
                    commands.trigger(ShowMore(false));
                },
            );
            more_item(
                font_assets,
                builder,
                "Reveal Puzzle",
                |_: Trigger<Pointer<Click>>, mut commands| {
                    commands.trigger(ShowMore(false));
                },
            );
            more_item(
                font_assets,
                builder,
                "Reset Puzzle",
                |_: Trigger<Pointer<Click>>, mut commands| {
                    commands.trigger(ShowMore(false));
                },
            );
        });
}

fn more_item(
    font_assets: &Res<FontAssets>,
    builder: &mut ChildBuilder,
    text: &str,
    trigger: fn(Trigger<Pointer<Click>>, Commands),
) {
    builder
        .spawn((
            Name::new("show-more-hint"),
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                width: Val::Px(153.0),
                height: Val::Px(56.0),
                padding: UiRect {
                    left: Val::Px(15.0),
                    right: Val::Px(15.0),
                    top: Val::Px(6.0),
                    bottom: Val::Px(4.0),
                },
                border: UiRect {
                    bottom: Val::Px(1.0),
                    ..default()
                },
                ..default()
            },
            BorderColor(*GRAY),
            GlobalZIndex(999),
        ))
        .with_children(|builder| {
            builder.spawn((
                Text::new(text),
                TextFont {
                    font_size: 18.0,
                    font: font_assets.franklin_500.clone(),
                    ..default()
                },
                TextColor(*DARK_BLACK),
            ));
        })
        .observe(trigger);
}

#[derive(Component)]
struct ShowMoreContainer;

fn on_show_more(
    trigger: Trigger<ShowMore>,
    mut q_more: Single<&mut Visibility, With<ShowMoreContainer>>,
) {
    let ShowMore(show_more) = trigger.event();
    if *show_more {
        **q_more = Visibility::Visible;
    } else {
        **q_more = Visibility::Hidden;
    }
}
