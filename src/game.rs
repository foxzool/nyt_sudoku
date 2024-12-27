use crate::game::dialog::ShowSettings;
use crate::{
    color::*,
    game::{
        board::ConflictContainer,
        board::{play_board, PreviewCandidate},
        cell_state::{
            AutoCandidates, CandidatesValue, CellMode, CellValueBundle, ConflictCell,
            CorrectionCell, DigitValueCell, FixedCell, ManualCandidates, RevealedCell,
            SelectedCell,
        },
        control_tab::control_board,
        dialog::{dialog_container, PauseGame, ShowHint},
        input::{keyboard_input, keyboard_move_cell},
        position::CellPosition,
    },
    loading::{FontAssets, TextureAssets},
    share::title_bar,
    GameState,
};
use bevy::{prelude::*, time::Stopwatch, utils::HashSet};
use sudoku::{
    bitset::Set,
    board::{CellState, Digit},
    strategy::StrategySolver,
    Sudoku,
};

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
            .init_resource::<Settings>()
            .add_event::<MoveSelectCell>()
            .add_event::<SudokuSolved>()
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
                    check_solver,
                    sudoku_solved,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_observer(on_new_digit)
            .add_observer(on_new_candidate)
            .add_observer(check_conflict)
            .add_observer(find_hint)
            .add_observer(on_clean_cell)
            .add_observer(on_select_cell)
            .add_observer(remove_conflict)
            .add_observer(on_unselect_cell)
            .add_observer(on_reset_puzzle)
            .add_observer(on_reveal_cell)
            .add_observer(on_reveal_puzzle)
            .add_observer(on_check_cell)
            .add_observer(on_check_puzzle)
            .add_observer(on_show_more);
    }
}

#[derive(Resource, Debug)]
pub struct SudokuManager {
    pub solution: Sudoku,
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
                        commands.trigger(ShowMore(true));
                    },
                );

            builder
                .spawn((
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
                ))
                .observe(
                    |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                        commands.trigger(ShowSettings(true));
                    },
                );
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
                    PauseButton,
                ))
                .observe(
                    |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                        commands.trigger(PauseGame(true));
                    },
                );
        });
}

#[derive(Component)]
struct PauseButton;

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

/// 手动候选格子容器
#[derive(Component)]
pub struct ManualCandidatesContainer;

/// 自动候选格子容器
#[derive(Component)]
pub struct AutoCandidatesContainer;

fn init_cells(
    mut commands: Commands,
    cell_background: Query<(Entity, &CellPosition)>,
    settings: Res<Settings>,
    mut auto: ResMut<AutoCandidateMode>,
) {
    let (sudoku, solution) = loop {
        let sudoku = Sudoku::generate();
        if let Some(solution) = sudoku.solution() {
            break (sudoku, solution);
        }
    };

    info!("sudoku: {:?}", sudoku);
    if settings.start_in_automatic_mode {
        *auto = AutoCandidateMode(true);
    }

    let solver = StrategySolver::from_sudoku(sudoku.clone());

    commands.insert_resource(SudokuManager {
        solution,
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
    trigger: Trigger<NewDigit>,
    mut q_cell: Query<
        (&mut DigitValueCell, &mut CellMode),
        (Without<FixedCell>, Without<RevealedCell>),
    >,
    mut commands: Commands,
    settings: Res<Settings>,
) {
    let entity = trigger.entity();
    let new_digit = trigger.event().0;
    if let Ok((mut cell_value, mut cell_mode)) = q_cell.get_mut(entity) {
        *cell_mode = CellMode::Digit;

        if let Some(old_digit) = cell_value.0 {
            if old_digit != new_digit {
                commands.trigger(RemoveDigit(old_digit));
            }
        }

        cell_value.0 = Some(new_digit);
        commands.trigger(CheckDigitConflict);

        if settings.check_guesses_when_entered {
            commands.trigger_targets(CheckCell, vec![entity]);
        }
    }
}

fn on_new_candidate(
    trigger: Trigger<NewCandidate>,
    mut q_cell: Query<
        (
            &mut DigitValueCell,
            &mut ManualCandidates,
            &mut AutoCandidates,
            &mut CellMode,
        ),
        (
            With<SelectedCell>,
            Without<FixedCell>,
            Without<RevealedCell>,
        ),
    >,
    auto_mode: Res<AutoCandidateMode>,
    mut commands: Commands,
) {
    let new_candidate = trigger.event().0;

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

fn on_clean_cell(
    trigger: Trigger<CleanCell>,
    mut q_cell: Query<
        (
            Entity,
            &mut DigitValueCell,
            &mut ManualCandidates,
            &mut CellMode,
        ),
        (Without<FixedCell>, Without<RevealedCell>),
    >,
    auto_mode: Res<AutoCandidateMode>,
    children: Query<&Children>,
    q_preview: Query<&PreviewCandidate>,
    mut commands: Commands,
) {
    if let Ok((entity, mut digit_value, mut manual_candidates, mut cell_mode)) =
        q_cell.get_mut(trigger.entity())
    {
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
        commands
            .entity(entity)
            .remove::<ConflictCell>()
            .remove::<CorrectionCell>();

        for child in children.iter_descendants(entity) {
            if let Ok(_preview) = q_preview.get(child) {
                commands.entity(child).remove::<PreviewCandidate>();
            }
        }
    }
}

fn check_solver(
    cell_query: Query<(&DigitValueCell, &CellPosition)>,
    sudoku_manager: Res<SudokuManager>,
    mut commands: Commands,
) {
    let mut solved_count = 0;
    for (cell_value, cell_position) in cell_query
        .iter()
        .sort_by::<&CellPosition>(|t1, t2| t1.0.cmp(&t2.0))
    {
        if let Some(digit) = cell_value.0 {
            for (index, num) in sudoku_manager.solution.iter().enumerate() {
                if cell_position.0 == index as u8 {
                    if Some(num.unwrap()) == Some(digit.get()) {
                        solved_count += 1;
                    }
                }
            }
        }

        if solved_count == 81 {
            commands.send_event(SudokuSolved);
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

/// 检查格子冲突
fn check_conflict(
    _check_digit: Trigger<CheckDigitConflict>,
    update_cell: Query<
        (Entity, &DigitValueCell, &CellPosition),
        (With<SelectedCell>, Without<FixedCell>),
    >,
    mut q_cell: Query<(
        Entity,
        &DigitValueCell,
        &CellPosition,
        Option<&mut ConflictCell>,
    )>,
    mut commands: Commands,
) {
    if let Ok((check_entity, digit_cell, cell_position)) = update_cell.get_single() {
        if let Some(check_digit) = digit_cell.0 {
            debug!("check conflict: {:?}", check_digit);
            let mut conflict_list = vec![];
            for (other_entity, other_cell_value, other_cell_position, opt_conflict) in
                q_cell.iter_mut()
            {
                if cell_position.in_range(other_cell_position) {
                    if let Some(other_digit) = other_cell_value.0 {
                        if check_digit == other_digit && cell_position != other_cell_position {
                            conflict_list.push(other_entity);
                            if let Some(mut conflict) = opt_conflict {
                                conflict.insert(check_entity);
                            } else {
                                commands
                                    .entity(other_entity)
                                    .insert(ConflictCell(HashSet::from([check_entity])));
                            }
                        }
                    }
                }
            }

            if !conflict_list.is_empty() {
                if let Ok((entity, _other_cell_value, _other_cell_position, opt_conflict)) =
                    q_cell.get_mut(check_entity)
                {
                    if let Some(mut conflict) = opt_conflict {
                        conflict.insert(check_entity);
                    } else {
                        commands
                            .entity(entity)
                            .insert(ConflictCell(HashSet::from_iter(conflict_list)));
                    }
                }
            }
        };
    }
}

fn show_conflict(
    mut q_conflict: Query<(Entity, &ConflictCell, &Children), Changed<ConflictCell>>,
    mut q_text: Query<&mut Text, With<ConflictContainer>>,
    mut commands: Commands,
) {
    for (entity, conflict, children) in q_conflict.iter_mut() {
        if conflict.is_empty() {
            commands.entity(entity).remove::<ConflictCell>();
        } else {
            for child in children.iter() {
                if let Ok(mut text) = q_text.get_mut(*child) {
                    text.0 = conflict.len().to_string();
                }
            }
        }
    }
}

fn remove_conflict(
    remove_digit: Trigger<RemoveDigit>,
    q_cell: Query<(Entity, &CellPosition), With<SelectedCell>>,
    mut other_cell: Query<
        (&DigitValueCell, &CellPosition, &mut ConflictCell),
        Without<SelectedCell>,
    >,
    mut commands: Commands,
) {
    let remove_digit = remove_digit.0;
    for (entity, cell_position) in q_cell.iter() {
        commands.entity(entity).remove::<ConflictCell>();

        for (other_cell_value, other_cell_position, mut conflict) in other_cell.iter_mut() {
            if cell_position.in_range(other_cell_position) {
                if let Some(other_digit) = other_cell_value.0 {
                    if remove_digit == other_digit && cell_position != other_cell_position {
                        conflict.remove(&entity);
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
    text: Single<(&mut Text, &mut Visibility), (With<TimerText>, Without<PauseButton>)>,
    mut pause_button: Single<&mut Visibility, (With<PauseButton>, Without<TimerText>)>,
    settings: Res<Settings>,
) {
    game_timer.tick(time.delta());
    let (mut text, mut visibility) = text.into_inner();
    text.0 = game_timer.to_string();
    if settings.show_clock {
        *visibility = Visibility::Visible;
        **pause_button = Visibility::Visible;
    } else {
        *visibility = Visibility::Hidden;
        **pause_button = Visibility::Hidden;
    }
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
                |_: Trigger<Pointer<Click>>, mut commands, _q_selected| {
                    commands.trigger(ShowMore(false));
                    commands.trigger(FindHint);
                },
            );
            more_item(
                font_assets,
                builder,
                "Check Cell",
                |_: Trigger<Pointer<Click>>, mut commands, q_selected| {
                    commands.trigger(ShowMore(false));
                    commands.trigger_targets(CheckCell, vec![*q_selected]);
                },
            );
            more_item(
                font_assets,
                builder,
                "Check Puzzle",
                |_: Trigger<Pointer<Click>>, mut commands, _q_selected| {
                    commands.trigger(ShowMore(false));
                    commands.trigger(CheckPuzzle);
                },
            );
            more_item(
                font_assets,
                builder,
                "Reveal Cell",
                |_: Trigger<Pointer<Click>>, mut commands, q_selected| {
                    commands.trigger(ShowMore(false));
                    commands.trigger_targets(RevealCell, vec![*q_selected]);
                },
            );
            more_item(
                font_assets,
                builder,
                "Reveal Puzzle",
                |_: Trigger<Pointer<Click>>, mut commands, _q_selected| {
                    commands.trigger(ShowMore(false));
                    commands.trigger(RevealPuzzle);
                },
            );
            more_item(
                font_assets,
                builder,
                "Reset Puzzle",
                |_: Trigger<Pointer<Click>>, mut commands, _q_selected| {
                    commands.trigger(ShowMore(false));
                    commands.trigger(ResetPuzzle);
                },
            );
        });
}

fn more_item(
    font_assets: &Res<FontAssets>,
    builder: &mut ChildBuilder,
    text: &str,
    trigger: fn(Trigger<Pointer<Click>>, Commands, Single<Entity, With<SelectedCell>>),
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
            BackgroundColor(WHITE_COLOR),
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
        .observe(
            |trigger: Trigger<Pointer<Over>>, mut item: Query<&mut BackgroundColor>| {
                let entity = trigger.entity();
                if let Ok(mut item) = item.get_mut(entity) {
                    item.0 = *EXTRA_LIGHT_GRAY;
                }
            },
        )
        .observe(
            |trigger: Trigger<Pointer<Out>>, mut item: Query<&mut BackgroundColor>| {
                let entity = trigger.entity();
                if let Ok(mut item) = item.get_mut(entity) {
                    item.0 = WHITE_COLOR;
                }
            },
        )
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

#[derive(Event)]
struct ResetPuzzle;

fn on_reset_puzzle(
    _trigger: Trigger<ResetPuzzle>,
    sudoku_manager: Res<SudokuManager>,
    mut q_cell: Query<(
        Entity,
        &CellPosition,
        &mut DigitValueCell,
        &mut ManualCandidates,
        &mut AutoCandidates,
        &mut CellMode,
    )>,
    mut commands: Commands,
    mut auto_mode: ResMut<AutoCandidateMode>,
) {
    commands.insert_resource(GameTimer(Stopwatch::new()));
    auto_mode.0 = false;
    let mut entities = vec![];
    for (entity, _, _, _, _, _) in q_cell.iter() {
        commands
            .entity(entity)
            .remove::<SelectedCell>()
            .remove::<ConflictCell>()
            .remove::<RevealedCell>();
        entities.push(entity);
    }

    commands.trigger_targets(CleanCell, entities);

    'l: for (index, cell_state) in sudoku_manager.solver.grid_state().into_iter().enumerate() {
        for (
            entity,
            cell_position,
            mut digit_value,
            mut manual_candidates,
            mut auto_candidates,
            mut cell_mode,
        ) in q_cell.iter_mut()
        {
            if cell_position.0 == index as u8 {
                if index == 0 {
                    commands.entity(entity).insert(SelectedCell);
                }
                match cell_state {
                    CellState::Digit(digit) => {
                        *cell_mode = CellMode::Digit;
                        digit_value.0 = Some(digit);
                        manual_candidates.0 = Set::NONE;
                        auto_candidates.0 = Set::NONE;
                    }
                    CellState::Candidates(cands) => {
                        *cell_mode = CellMode::ManualCandidates;
                        digit_value.0 = None;
                        manual_candidates.0 = Set::NONE;
                        auto_candidates.0 = cands;
                    }
                }

                continue 'l;
            }
        }
    }
}

#[derive(Event)]
struct RevealCell;

fn on_reveal_cell(
    trigger: Trigger<RevealCell>,
    q_select: Query<&CellPosition>,
    sudoku_manager: Res<SudokuManager>,
    mut commands: Commands,
) {
    let entity = trigger.entity();

    if let Ok(cell_position) = q_select.get(entity) {
        for (index, num) in sudoku_manager.solution.iter().enumerate() {
            if cell_position.0 == index as u8 {
                let num = num.unwrap();
                commands.trigger_targets(NewDigit::new(num), vec![entity]);
                commands.entity(entity).insert(RevealedCell);
                return;
            }
        }
    }
}

#[derive(Event)]
struct RevealPuzzle;

fn on_reveal_puzzle(
    _trigger: Trigger<RevealPuzzle>,
    q_cell: Query<Entity, (Without<FixedCell>, With<DigitValueCell>)>,
    mut commands: Commands,
    mut auto: ResMut<AutoCandidateMode>,
    settings: Res<Settings>,
) {
    if settings.start_in_automatic_mode {
        *auto = AutoCandidateMode(true);
    }

    let entities = q_cell.iter().collect::<Vec<_>>();
    commands.trigger_targets(RevealCell, entities);
}

#[derive(Event)]
pub struct SudokuSolved;

fn sudoku_solved(mut ev: EventReader<SudokuSolved>, mut time: ResMut<Time<Virtual>>) {
    for _ in ev.read() {
        time.pause();
    }
}

#[derive(Event)]
pub struct CheckCell;

fn on_check_cell(
    trigger: Trigger<CheckCell>,
    q_cell: Query<(&DigitValueCell, &CellPosition), Without<FixedCell>>,
    sudoku_manager: Res<SudokuManager>,
    mut commands: Commands,
    settings: Res<Settings>,
) {
    let entity = trigger.entity();
    if let Ok((cell_value, cell_position)) = q_cell.get(entity) {
        if let Some(digit) = cell_value.0 {
            for (index, num) in sudoku_manager.solution.iter().enumerate() {
                if cell_position.0 == index as u8 {
                    if num != Some(digit.get()) {
                        commands.entity(entity).insert(CorrectionCell);
                    } else {
                        commands.entity(entity).remove::<CorrectionCell>();

                        if settings.check_guesses_when_entered {
                            commands.entity(entity).insert(RevealedCell);
                        }
                    }
                }
            }
        }
    }
}

#[derive(Event)]
pub struct CheckPuzzle;

fn on_check_puzzle(
    _trigger: Trigger<CheckPuzzle>,
    q_cell: Query<(Entity, &DigitValueCell, &CellPosition), Without<FixedCell>>,
    sudoku_manager: Res<SudokuManager>,
    mut commands: Commands,
) {
    for (entity, cell_value, cell_position) in q_cell.iter() {
        if let Some(digit) = cell_value.0 {
            for (index, num) in sudoku_manager.solution.iter().enumerate() {
                if cell_position.0 == index as u8 {
                    if num != Some(digit.get()) {
                        commands.entity(entity).insert(CorrectionCell);
                    }
                }
            }
        }
    }
}

#[derive(Event)]
pub struct FindHint;

/// 查找提示, 暂时按照候选数最少的格子来选中
fn find_hint(
    _trigger: Trigger<FindHint>,
    q_selected: Query<Entity, With<SelectedCell>>,
    q_cell: Query<(Entity, &AutoCandidates), Without<FixedCell>>,
    mut commands: Commands,
) {
    for entity in q_selected.iter() {
        commands.entity(entity).remove::<SelectedCell>();
    }
    if let Some((entity, _)) = q_cell
        .iter()
        .sort_by::<(Entity, &AutoCandidates)>(|t1, t2| {
            let candidate_1 = t1.1;
            let candidate_2 = t2.1;

            candidate_1.0.len().cmp(&candidate_2.0.len())
        })
        .into_iter()
        .next()
    {
        commands.entity(entity).insert(SelectedCell);
    }
}

#[derive(Resource)]
pub struct Settings {
    pub check_guesses_when_entered: bool,
    pub start_in_automatic_mode: bool,
    pub highlight_conflicts: bool,
    pub play_sound_on_solve: bool,
    pub show_clock: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            check_guesses_when_entered: false,
            start_in_automatic_mode: false,
            highlight_conflicts: true,
            play_sound_on_solve: true,
            show_clock: true,
        }
    }
}
