use crate::game::cell_state::{ConflictCell, CorrectionCell};
use crate::{
    color::*,
    game::cell_state::{
        AutoCandidateCellMarker, CandidateMarker, CandidatesValue, ManualCandidateCellMarker,
        RevealedCell,
    },
    game::cell_state::{AutoCandidates, CellMode, DigitValueCell, ManualCandidates},
    game::position::CellPosition,
    game::{
        AutoCandidateMode, AutoCandidatesContainer, DigitCellContainer, ManualCandidatesContainer,
        MoveSelectCell, SelectedCell,
    },
    loading::{FontAssets, TextureAssets},
    GameState,
};
use bevy::prelude::*;
use sudoku::board::Digit;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            show_digit_cell,
            show_candidates::<AutoCandidates, AutoCandidateCellMarker>,
            show_candidates::<ManualCandidates, ManualCandidateCellMarker>,
            show_preview_number,
            change_cell_vis,
        )
            .run_if(in_state(GameState::Playing)),
    )
    .add_systems(
        Update,
        switch_candidate_cell_mode.run_if(resource_changed::<AutoCandidateMode>),
    )
    .add_observer(move_select_cell)
    .add_observer(on_insert_conflict)
    .add_observer(on_remove_conflict)
    .add_observer(on_remove_correction)
    .add_observer(on_insert_correction)

    ;
}

pub(crate) fn play_board(
    font_assets: &Res<FontAssets>,
    texture_assets: &Res<TextureAssets>,
    builder: &mut ChildBuilder,
) {
    builder
        .spawn((
            Node {
                width: Val::Vh(80.0),
                // min_width: Val::Px(500.0),
                // max_width: Val::Px(800.0),
                ..default()
            },
            BackgroundColor(*DARK_BLACK),
        ))
        .with_children(|builder| {
            // 生成9宫格布局
            builder
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        aspect_ratio: Some(1.0),
                        display: Display::Grid,
                        grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
                        grid_template_rows: RepeatedGridTrack::flex(3, 1.0),
                        row_gap: Val::Px(4.0),
                        column_gap: Val::Px(4.0),
                        border: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    // BorderColor(Color::BLACK),
                    BackgroundColor(*GRAY),
                    // CellsLayout,
                ))
                .with_children(|builder| {
                    // 生成九个宫格
                    for block_index in 0..9 {
                        builder
                            .spawn((
                                Node {
                                    height: Val::Percent(100.0),
                                    aspect_ratio: Some(1.0),
                                    display: Display::Grid,
                                    grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
                                    grid_template_rows: RepeatedGridTrack::flex(3, 1.0),
                                    // row_gap: Val::Px(1.0),
                                    // column_gap: Val::Px(1.0),
                                    // border: UiRect::all(Val::Px(1.)),
                                    ..default()
                                },
                                BackgroundColor(*GRAY),
                            ))
                            .with_children(|builder| {
                                // 生成宫格里的9个格子
                                for bi in 0..9 {
                                    let cell = block_index * 9 + bi;
                                    builder
                                        .spawn((
                                            Node {
                                                display: Display::Grid,
                                                align_items: AlignItems::Center,
                                                justify_items: JustifyItems::Center,
                                                align_content: AlignContent::Center,
                                                justify_content: JustifyContent::Center,
                                                border: UiRect::all(Val::Px(0.5)),
                                                ..default()
                                            },
                                            CellPosition::from_block_row_col(block_index, bi),
                                            BorderColor(*LIGHT_GRAY),
                                            BackgroundColor(Color::WHITE),
                                        ))
                                        .observe(on_click_cell)
                                        .with_children(|builder| {
                                            // 数字格子
                                            builder.spawn((
                                                Text::new(cell.to_string()),
                                                TextFont {
                                                    font: font_assets.franklin_800.clone(),
                                                    font_size: 48.0,
                                                    ..default()
                                                },
                                                TextColor(*DARK_BLACK),
                                                Visibility::Hidden,
                                                Node {
                                                    margin: UiRect {
                                                        bottom: Val::Px(1.0),
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                DigitCellContainer,
                                            ));

                                            // 自动候选格子容器
                                            builder
                                                .spawn((
                                                    Visibility::Hidden,
                                                    Node {
                                                        height: Val::Percent(100.0),
                                                        display: Display::Grid,
                                                        aspect_ratio: Some(1.0),
                                                        position_type: PositionType::Absolute,
                                                        grid_template_columns:
                                                            RepeatedGridTrack::flex(3, 1.0),
                                                        grid_template_rows: RepeatedGridTrack::flex(
                                                            3, 1.0,
                                                        ),
                                                        // row_gap: Val::Px(4.0),
                                                        // column_gap: Val::Px(4.0),
                                                        ..default()
                                                    },
                                                    AutoCandidatesContainer,
                                                ))
                                                .with_children(|builder| {
                                                    // 9个候选数字格子
                                                    for i in 1..=9u8 {
                                                        builder
                                                            .spawn((
                                                                Text::new(i.to_string()),
                                                                TextFont {
                                                                    font: font_assets
                                                                        .franklin_700
                                                                        .clone(),
                                                                    font_size: 16.0,
                                                                    ..default()
                                                                },
                                                                TextColor(TRANSPARENT),
                                                                TextLayout::new_with_justify(
                                                                    JustifyText::Center,
                                                                ),
                                                                Node {
                                                                    align_items: AlignItems::Center,
                                                                    justify_items:
                                                                        JustifyItems::Center,
                                                                    align_content:
                                                                        AlignContent::Center,
                                                                    justify_content:
                                                                        JustifyContent::Center,
                                                                    // margin: UiRect {
                                                                    //     top: Val::Px(4.),
                                                                    //     ..default()
                                                                    // },
                                                                    ..default()
                                                                },
                                                                Visibility::Inherited,
                                                                // BackgroundColor(RED.into()),
                                                                AutoCandidateCellMarker {
                                                                    index: i,
                                                                    selected: false,
                                                                },
                                                            ))
                                                            .observe(
                                                                candidate_cell_move::<
                                                                    AutoCandidates,
                                                                    AutoCandidateCellMarker,
                                                                >,
                                                            )
                                                            .observe(
                                                                candidate_cell_out::<
                                                                    AutoCandidateCellMarker,
                                                                >,
                                                            )
                                                            .observe(
                                                                candidate_cell_click::<
                                                                    AutoCandidates,
                                                                    AutoCandidateCellMarker,
                                                                >,
                                                            );
                                                    }
                                                });

                                            // 手动候选格子容器
                                            builder
                                                .spawn((
                                                    Visibility::Hidden,
                                                    Node {
                                                        height: Val::Percent(100.0),
                                                        display: Display::Grid,
                                                        aspect_ratio: Some(1.0),
                                                        position_type: PositionType::Absolute,
                                                        grid_template_columns:
                                                            RepeatedGridTrack::flex(3, 1.0),
                                                        grid_template_rows: RepeatedGridTrack::flex(
                                                            3, 1.0,
                                                        ),
                                                        // row_gap: Val::Px(4.0),
                                                        // column_gap: Val::Px(4.0),
                                                        ..default()
                                                    },
                                                    ManualCandidatesContainer,
                                                ))
                                                .with_children(|builder| {
                                                    // 9个候选数字格子
                                                    for i in 1..=9u8 {
                                                        builder
                                                            .spawn((
                                                                Text::new(i.to_string()),
                                                                TextFont {
                                                                    font: font_assets
                                                                        .franklin_700
                                                                        .clone(),
                                                                    font_size: 16.0,
                                                                    ..default()
                                                                },
                                                                TextColor(TRANSPARENT),
                                                                TextLayout::new_with_justify(
                                                                    JustifyText::Center,
                                                                ),
                                                                Node {
                                                                    align_items: AlignItems::Center,
                                                                    justify_items:
                                                                        JustifyItems::Center,
                                                                    align_content:
                                                                        AlignContent::Center,
                                                                    justify_content:
                                                                        JustifyContent::Center,
                                                                    margin: UiRect {
                                                                        top: Val::Px(4.),
                                                                        ..default()
                                                                    },
                                                                    ..default()
                                                                },
                                                                Visibility::Inherited,
                                                                // BackgroundColor(YELLOW.into()),
                                                                ManualCandidateCellMarker {
                                                                    index: i,
                                                                    selected: false,
                                                                },
                                                            ))
                                                            .observe(
                                                                candidate_cell_move::<
                                                                    ManualCandidates,
                                                                    ManualCandidateCellMarker,
                                                                >,
                                                            )
                                                            .observe(
                                                                candidate_cell_out::<
                                                                    ManualCandidateCellMarker,
                                                                >,
                                                            )
                                                            .observe(
                                                                candidate_cell_click::<
                                                                    ManualCandidates,
                                                                    ManualCandidateCellMarker,
                                                                >,
                                                            );
                                                    }
                                                });
                                        });
                                }
                            });
                    }
                });
        });
}

fn spawn_conflict_container(texture_assets: &Res<TextureAssets>, builder: &mut ChildBuilder) {
    builder.spawn((
        ImageNode {
            image: texture_assets.circle.clone(),
            color: Color::srgb_u8(255, 75, 86),
            ..default()
        },
        // Visibility::Hidden,
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(7.0),
            bottom: Val::Px(7.0),
            width: Val::Px(14.0),
            height: Val::Px(14.0),
            ..default()
        },
        ConflictContainer,
    ));
}

#[derive(Component)]
pub struct PreviewCandidate {
    hold: bool,
    timer: Timer,
}

impl Default for PreviewCandidate {
    fn default() -> Self {
        Self {
            hold: false,
            timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}

impl PreviewCandidate {
    pub fn hold() -> Self {
        Self {
            hold: true,
            timer: Timer::from_seconds(0.5, TimerMode::Once),
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

fn show_digit_cell(
    q_cell: Query<(Entity, &DigitValueCell, &CellMode, Option<&RevealedCell>)>,
    children: Query<&Children>,
    mut digit_cell: Query<(&mut Text, &mut Visibility, &mut TextColor), With<DigitCellContainer>>,
) {
    for (entity, digit_value, cell_mode, opt_revealed) in q_cell.iter() {
        for child in children.iter_descendants(entity) {
            if let Ok((mut text, mut visibility, mut text_color)) = digit_cell.get_mut(child) {
                if let CellMode::Digit = cell_mode {
                    if let Some(digit) = digit_value.0 {
                        text.0 = digit.get().to_string();
                    }
                    *visibility = Visibility::Visible;
                    if opt_revealed.is_some() {
                        text_color.0 = *ACCENT_BLUE;
                    } else {
                        text_color.0 = *DARK_BLACK;
                    }
                } else {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}

fn show_candidates<C: CandidatesValue, M: CandidateMarker>(
    q_cell: Query<(Entity, &C)>,
    children: Query<&Children>,
    mut candidate_cell: Query<(&mut TextColor, &mut M)>,
) {
    for (entity, manual_candidates) in q_cell.iter() {
        for child in children.iter_descendants(entity) {
            if let Ok((mut text_color, mut cell_marker)) = candidate_cell.get_mut(child) {
                if manual_candidates
                    .candidates()
                    .contains(Digit::new(cell_marker.index()).as_set())
                {
                    cell_marker.set_selected(true);
                    *text_color = TextColor(*GRAY2);
                } else {
                    cell_marker.set_selected(false);
                    *text_color = TextColor(TRANSPARENT);
                }
            }
        }
    }
}

fn show_preview_number(
    mut candidate_cell: Query<(Entity, &mut TextColor, &mut PreviewCandidate)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut text_color, mut preview) in candidate_cell.iter_mut() {
        if preview.hold {
            *text_color = TextColor(*GRAY);
            continue;
        }
        *text_color = TextColor(*LIGHTER_GRAY);

        preview.timer.tick(time.delta());
        let alpha = 1.5 - preview.timer.elapsed_secs();
        text_color.0.set_alpha(alpha);

        if preview.timer.just_finished() {
            commands.entity(entity).remove::<PreviewCandidate>();
            *text_color = TextColor(TRANSPARENT)
        }
    }
}

fn candidate_cell_click<C: CandidatesValue, M: CandidateMarker>(
    click: Trigger<Pointer<Click>>,
    cell: Query<&M>,
    parent_query: Query<&Parent>,
    mut q_select: Query<&mut C, With<SelectedCell>>,
    mut commands: Commands,
) {
    let candidate_cell = cell.get(click.entity()).unwrap();
    for ancestor in parent_query.iter_ancestors(click.entity()) {
        if let Ok(mut cell_value) = q_select.get_mut(ancestor) {
            cell_value.insert(Digit::new(candidate_cell.index()));

            commands.entity(click.entity()).remove::<PreviewCandidate>();
        }
    }
}

fn candidate_cell_move<C: CandidatesValue, M: CandidateMarker>(
    trigger: Trigger<Pointer<Over>>,
    cell: Query<&M>,
    parent_query: Query<&Parent>,
    q_select: Query<&mut C, With<SelectedCell>>,
    mut commands: Commands,
) {
    if let Ok(manual_marker) = cell.get(trigger.entity()) {
        for ancestor in parent_query.iter_ancestors(trigger.entity()) {
            if let Ok(_cell_value) = q_select.get(ancestor) {
                if !manual_marker.selected() {
                    commands
                        .entity(trigger.entity())
                        .insert(PreviewCandidate::hold());
                }
            }
        }
    }
}

fn candidate_cell_out<M: CandidateMarker>(
    trigger: Trigger<Pointer<Out>>,
    cell: Query<&M>,
    mut commands: Commands,
    parent_query: Query<&Parent>,
    q_select: Query<&CellMode, With<SelectedCell>>,
) {
    if let Ok(manual_marker) = cell.get(trigger.entity()) {
        for ancestor in parent_query.iter_ancestors(trigger.entity()) {
            if let Ok(cell_mode) = q_select.get(ancestor) {
                if *cell_mode != CellMode::Digit {
                    if !manual_marker.selected() {
                        commands
                            .entity(trigger.entity())
                            .insert(PreviewCandidate::default());
                    }
                }
            }
        }
    }
}

fn move_select_cell(
    move_ev: Trigger<MoveSelectCell>,
    mut commands: Commands,
    q_select: Single<(Entity, &CellPosition), With<SelectedCell>>,
    q_other: Query<(Entity, &CellPosition), Without<SelectedCell>>,
) {
    let (entity, cell_position) = q_select.into_inner();
    let new_position = match move_ev.event() {
        MoveSelectCell::Up if cell_position.row() > 0 => Some(CellPosition::from_row_col(
            cell_position.row() - 1,
            cell_position.col(),
        )),
        MoveSelectCell::Down if cell_position.row() < 8 => Some(CellPosition::from_row_col(
            cell_position.row() + 1,
            cell_position.col(),
        )),
        MoveSelectCell::Left if cell_position.col() > 0 => Some(CellPosition::from_row_col(
            cell_position.row(),
            cell_position.col() - 1,
        )),
        MoveSelectCell::Right if cell_position.col() < 8 => Some(CellPosition::from_row_col(
            cell_position.row(),
            cell_position.col() + 1,
        )),
        _ => None,
    };

    if let Some(new_position) = new_position {
        commands.entity(entity).remove::<SelectedCell>();
        for (other_entity, cell_position) in q_other.iter() {
            if cell_position == &new_position {
                commands.entity(other_entity).insert(SelectedCell);
                break;
            }
        }
    }
}

fn switch_candidate_cell_mode(
    auto_mode: Res<AutoCandidateMode>,
    mut q_cell_mode: Query<&mut CellMode>,
) {
    for mut cell_mode in q_cell_mode.iter_mut() {
        if *cell_mode != CellMode::Digit {
            if **auto_mode {
                *cell_mode = CellMode::AutoCandidates;
            } else {
                *cell_mode = CellMode::ManualCandidates;
            }
        }
    }
}

fn change_cell_vis(
    q_cell: Query<(&CellMode, &Children), Changed<CellMode>>,
    mut q_digit: Query<
        &mut Visibility,
        (
            With<DigitCellContainer>,
            Without<ConflictCell>,
            Without<ManualCandidatesContainer>,
            Without<AutoCandidatesContainer>,
        ),
    >,
    mut q_manual: Query<
        &mut Visibility,
        (
            With<ManualCandidatesContainer>,
            Without<DigitCellContainer>,
            Without<ConflictCell>,
            Without<AutoCandidatesContainer>,
        ),
    >,
    mut q_auto: Query<
        &mut Visibility,
        (
            With<AutoCandidatesContainer>,
            Without<DigitCellContainer>,
            Without<ConflictCell>,
            Without<ManualCandidatesContainer>,
        ),
    >,
) {
    for (cell_mode, children) in q_cell.iter() {
        match cell_mode {
            CellMode::Digit => {
                change_vis_inner(
                    children,
                    &mut q_digit,
                    &mut q_manual,
                    &mut q_auto,
                    Visibility::Visible,
                    Visibility::Hidden,
                    Visibility::Hidden,
                );
            }
            CellMode::ManualCandidates => {
                change_vis_inner(
                    children,
                    &mut q_digit,
                    &mut q_manual,
                    &mut q_auto,
                    Visibility::Hidden,
                    Visibility::Visible,
                    Visibility::Hidden,
                );
            }
            CellMode::AutoCandidates => {
                change_vis_inner(
                    children,
                    &mut q_digit,
                    &mut q_manual,
                    &mut q_auto,
                    Visibility::Hidden,
                    Visibility::Hidden,
                    Visibility::Visible,
                );
            }
        }
    }
}

fn change_vis_inner(
    children: &Children,
    q_digit: &mut Query<
        &mut Visibility,
        (
            With<DigitCellContainer>,
            Without<ConflictCell>,
            Without<ManualCandidatesContainer>,
            Without<AutoCandidatesContainer>,
        ),
    >,
    q_manual: &mut Query<
        &mut Visibility,
        (
            With<ManualCandidatesContainer>,
            Without<DigitCellContainer>,
            Without<ConflictCell>,
            Without<AutoCandidatesContainer>,
        ),
    >,
    q_auto: &mut Query<
        &mut Visibility,
        (
            With<AutoCandidatesContainer>,
            Without<DigitCellContainer>,
            Without<ConflictCell>,
            Without<ManualCandidatesContainer>,
        ),
    >,
    digit_vis: Visibility,
    manual_vis: Visibility,
    auto_vis: Visibility,
) {
    for child in children.iter() {
        if let Ok(mut vis) = q_digit.get_mut(*child) {
            *vis = digit_vis;
        }
        if let Ok(mut vis) = q_manual.get_mut(*child) {
            *vis = manual_vis;
        }
        if let Ok(mut vis) = q_auto.get_mut(*child) {
            *vis = auto_vis;
        }
    }
}

#[derive(Component)]
pub struct ConflictContainer;

fn on_insert_conflict(
    trigger: Trigger<OnInsert, ConflictCell>,
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
) {
    commands.entity(trigger.entity()).with_children(|builder| {
        spawn_conflict_container(&texture_assets, builder);
    });
}

fn on_remove_conflict(
    trigger: Trigger<OnRemove, ConflictCell>,
    mut commands: Commands,
    children: Query<&Children>,
    q_conflict: Query<Entity, With<ConflictContainer>>,
) {
    for child in children.iter_descendants(trigger.entity()) {
        if let Ok(conflict) = q_conflict.get(child) {
            commands.entity(conflict).despawn_recursive();
        }
    }
}

fn on_insert_correction(
    trigger: Trigger<OnInsert, CorrectionCell>,
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
) {
    commands.entity(trigger.entity()).with_children(|builder| {
        spawn_correction_container(&texture_assets, builder);
    });
}

fn on_remove_correction(
    trigger: Trigger<OnRemove, CorrectionCell>,
    mut commands: Commands,
    children: Query<&Children>,
    q_correction: Query<Entity, With<CorrectionContainer>>,
) {
    for child in children.iter_descendants(trigger.entity()) {
        if let Ok(correction) = q_correction.get(child) {
            commands.entity(correction).despawn_recursive();
        }
    }
}

#[derive(Component)]
struct CorrectionContainer;

fn spawn_correction_container(texture_assets: &Res<TextureAssets>, builder: &mut ChildBuilder) {
    builder.spawn((
        ImageNode {
            image: texture_assets.correction.clone(),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        CorrectionContainer,
    ));
}
