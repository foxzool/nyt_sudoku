use crate::color::*;
use crate::game::cell_state::{
    AutoCandidates, CellMode, CellValue, DigitValueCell, FixedCell, ManualCandidates,
};
use crate::game::position::CellPosition;
use crate::game::{
    AutoCandidateCellMarker, AutoCandidateMode, CandidateCell, CandidatesContainer, CellGrid,
    ConflictCount, DigitCellMarker, ManualCandidateCellMarker, MoveSelectCell, SelectedCell,
};
use crate::GameState;
use bevy::prelude::*;
use bevy::utils::info;
use std::ops::{BitOrAssign, BitXorAssign};
use sudoku::board::{CellState, Digit};

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            show_digit_cell,
            show_manual_candidates,
            show_auto_candidates,
            show_preview_number,
            move_select_cell,
        )
            .run_if(in_state(GameState::Playing)),
    );
}

pub(crate) fn play_board(asset_server: &Res<AssetServer>, builder: &mut ChildBuilder) {
    builder
        .spawn((
            Node {
                width: Val::Vh(80.0),
                // min_width: Val::Px(500.0),
                // max_width: Val::Px(800.0),
                ..default()
            }, BackgroundColor(*DARK_BLACK),
        )).with_children(|builder| {
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
                                row_gap: Val::Px(1.0),
                                column_gap: Val::Px(1.0),
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
                                    .spawn((Node {
                                        display: Display::Grid,
                                        ..default()
                                    },))
                                    .with_children(|builder| {
                                        // 格子
                                        builder
                                            .spawn((
                                                CellPosition::from_block_row_col(block_index, bi),
                                                Node {
                                                    align_items: AlignItems::Center,
                                                    justify_items: JustifyItems::Center,
                                                    align_content: AlignContent::Center,
                                                    justify_content: JustifyContent::Center,
                                                    ..default()
                                                },
                                                BackgroundColor(Color::WHITE),
                                                CellGrid,
                                            ))
                                            .observe(on_click_cell)
                                            .with_children(|builder| {
                                                // 数字格子
                                                builder.spawn((
                                                    Text::new(cell.to_string()),
                                                    TextFont {
                                                        font: asset_server.load("fonts/franklin-normal-800.ttf"),
                                                        font_size: 48.0,
                                                        ..default()
                                                    },
                                                    TextColor(Color::srgb_u8(18, 18, 18)),
                                                    Visibility::Hidden,
                                                    Node {
                                                        margin: UiRect {
                                                            bottom: Val::Px(1.0),
                                                            ..default()
                                                        },
                                                        ..default()
                                                    },
                                                    DigitCellMarker,
                                                ));

                                                // 冲突计数器
                                                builder.spawn((
                                                    ImageNode {
                                                        image: asset_server.load("textures/circle.png"),
                                                        color: Color::srgb_u8(255, 75, 86),
                                                        ..default()
                                                    },
                                                    Visibility::Hidden,
                                                    Node {
                                                        position_type: PositionType::Absolute,
                                                        right: Val::Px(7.0),
                                                        bottom: Val::Px(7.0),
                                                        width: Val::Px(14.0),
                                                        height: Val::Px(14.0),
                                                        ..default()
                                                    },
                                                    ConflictCount::default()
                                                )
                                                );

                                                // 候选格子容器
                                                builder
                                                    .spawn((
                                                        // Visibility::Hidden,
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
                                                        CandidatesContainer,
                                                    ))
                                                    .with_children(|builder| {
                                                        // 9个候选数字格子
                                                        for i in 1..=9u8 {
                                                            builder
                                                                .spawn((
                                                                    Text::new(i.to_string()),
                                                                    TextFont {
                                                                        font: asset_server.load("fonts/franklin-normal-700.ttf"),
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
                                                                    // Visibility::Hidden,
                                                                    // BackgroundColor(Color::WHITE),
                                                                    AutoCandidateCellMarker {
                                                                        index: i,
                                                                        selected: false,
                                                                    },
                                                                ))
                                                                .observe(manual_candidate_cell_move)
                                                                .observe(manual_candidate_cell_out)
                                                                .observe(manual_candidate_cell_click);
                                                        }

                                                        for i in 1..=9u8 {
                                                            builder
                                                                .spawn((
                                                                    Text::new(i.to_string()),
                                                                    TextFont {
                                                                        font: asset_server.load("fonts/franklin-normal-700.ttf"),
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
                                                                    // Visibility::Hidden,
                                                                    // BackgroundColor(Color::WHITE),
                                                                    ManualCandidateCellMarker {
                                                                        index: i,
                                                                        selected: false,
                                                                    },
                                                                ))
                                                                .observe(manual_candidate_cell_move)
                                                                .observe(manual_candidate_cell_out)
                                                                .observe(manual_candidate_cell_click);
                                                        }
                                                    });
                                            });
                                        // builder.spawn((Node::default(), BackgroundColor(Color::WHITE)));
                                    });
                            }
                        });
                }
            });
    });
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
    q_cell: Query<(Entity, &DigitValueCell, &CellMode)>,
    children: Query<&Children>,
    mut digit_cell: Query<(&mut Text, &mut Visibility), With<DigitCellMarker>>,
) {
    for (entity, digit_value, cell_mode) in q_cell.iter() {
        for child in children.iter_descendants(entity) {
            if let Ok((mut text, mut visibility)) = digit_cell.get_mut(child) {
                if let CellMode::Digit = cell_mode {
                    if let Some(digit) = digit_value.0 {
                        text.0 = digit.get().to_string();
                    }
                    *visibility = Visibility::Visible;
                } else {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}

fn show_manual_candidates(
    q_cell: Query<(Entity, &ManualCandidates, &CellMode)>,
    children: Query<&Children>,
    mut candidate_cell: Query<(&mut TextColor, &mut ManualCandidateCellMarker)>,
) {
    for (entity, manual_candidates, cell_mode) in q_cell.iter() {
        for child in children.iter_descendants(entity) {
            if let Ok((mut text_color, mut cell_marker)) = candidate_cell.get_mut(child) {
                if let CellMode::ManualCandidates = cell_mode {
                    if manual_candidates
                        .0
                        .contains(Digit::new(cell_marker.index).as_set())
                    {
                        cell_marker.selected = true;
                        *text_color = TextColor(*GRAY2);
                    } else {
                        cell_marker.selected = false;
                        *text_color = TextColor(TRANSPARENT);
                    }
                } else {
                    *text_color = TextColor(TRANSPARENT);
                }
            }
        }
    }
}

fn show_auto_candidates(
    q_cell: Query<(Entity, &AutoCandidates, &CellMode)>,
    children: Query<&Children>,
    mut candidate_cell: Query<(&mut TextColor, &mut AutoCandidateCellMarker)>,
) {
    for (entity, auto_candidates, cell_mode) in q_cell.iter() {
        for child in children.iter_descendants(entity) {
            if let Ok((mut text_color, mut cell_marker)) = candidate_cell.get_mut(child) {
                if let CellMode::AutoCandidates = cell_mode {
                    if auto_candidates
                        .0
                        .contains(Digit::new(cell_marker.index).as_set())
                    {
                        cell_marker.selected = true;
                        *text_color = TextColor(*GRAY2);
                    } else {
                        cell_marker.selected = false;
                        *text_color = TextColor(TRANSPARENT);
                    }
                } else {
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

fn update_cell_number(
    cell: Query<(&CellValue, &Children, Option<&FixedCell>, &CellPosition)>,
    auto_mode: Res<AutoCandidateMode>,
    mut digit_cell: Query<
        (&mut Text, &mut Visibility),
        (With<DigitCellMarker>, Without<CandidatesContainer>),
    >,
    mut candidates_container: Query<
        (&mut Visibility, &Children),
        (With<CandidatesContainer>, Without<DigitCellMarker>),
    >,
    mut candidate_cell: Query<
        (
            &mut TextColor,
            &mut CandidateCell,
            Option<&mut PreviewCandidate>,
        ),
        (Without<DigitCellMarker>, Without<CandidatesContainer>),
    >,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (cell_value, children, opt_fixed, cell_position) in cell.iter() {
        for child in children.iter() {
            if let Ok((_text, mut visibility)) = digit_cell.get_mut(*child) {
                *visibility = Visibility::Hidden;
            }
            if let Ok((mut visibility, _children)) = candidates_container.get_mut(*child) {
                *visibility = Visibility::Hidden;
            }
            match cell_value.current(**auto_mode) {
                CellState::Digit(digit) => {
                    if let Ok((mut text, mut visibility)) = digit_cell.get_mut(*child) {
                        debug!("cell {} change to digit {}", cell_position, digit.get());
                        text.0 = digit.get().to_string();
                        *visibility = Visibility::Visible;
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
                            if let Ok((mut text_color, mut cell, opt_preview)) =
                                candidate_cell.get_mut(*child)
                            {
                                if candidates.contains(Digit::new(cell.index).as_set()) {
                                    if **auto_mode {
                                        cell.auto_candidate_selected = true;
                                        cell.manual_candidate_selected = false;
                                    } else {
                                        cell.manual_candidate_selected = true;
                                        cell.auto_candidate_selected = false;
                                    }

                                    *text_color = TextColor(*GRAY);
                                }
                                // 处理预览候选数字
                                else if let Some(mut preview) = opt_preview {
                                    *text_color = TextColor(*LIGHTER_GRAY);
                                    if !preview.hold {
                                        preview.timer.tick(time.delta());
                                        let alpha = 1.5 - preview.timer.elapsed_secs();
                                        text_color.0.set_alpha(alpha);

                                        if preview.timer.just_finished() {
                                            commands.entity(*child).remove::<PreviewCandidate>();
                                            *text_color = TextColor(TRANSPARENT)
                                        }
                                    }
                                } else {
                                    if **auto_mode {
                                        cell.auto_candidate_selected = false;
                                    } else {
                                        cell.manual_candidate_selected = false;
                                    }
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

fn manual_candidate_cell_click(
    click: Trigger<Pointer<Click>>,
    mut cell: Query<&mut ManualCandidateCellMarker>,
    parent_query: Query<&Parent>,
    mut q_select: Query<&mut ManualCandidates, With<SelectedCell>>,
    mut commands: Commands,
) {
    let mut candidate_cell = cell.get_mut(click.entity()).unwrap();
    candidate_cell.selected = !candidate_cell.selected;
    for ancestor in parent_query.iter_ancestors(click.entity()) {
        if let Ok(mut cell_value) = q_select.get_mut(ancestor) {
            cell_value.insert(Digit::new(candidate_cell.index));

            commands.entity(click.entity()).remove::<PreviewCandidate>();
        }
    }
}

fn manual_candidate_cell_move(
    trigger: Trigger<Pointer<Over>>,
    mut cell: Query<&ManualCandidateCellMarker>,
    parent_query: Query<&Parent>,
    mut q_select: Query<&mut ManualCandidates, With<SelectedCell>>,
    mut commands: Commands,
) {
    if let Ok(manual_marker) = cell.get(trigger.entity()) {
        for ancestor in parent_query.iter_ancestors(trigger.entity()) {
            if let Ok(_cell_value) = q_select.get(ancestor) {
                if !manual_marker.selected {
                    commands
                        .entity(trigger.entity())
                        .insert(PreviewCandidate::hold());
                }
            }
        }
    }
}

fn manual_candidate_cell_out(
    trigger: Trigger<Pointer<Out>>,
    mut cell: Query<&ManualCandidateCellMarker>,
    mut commands: Commands,
    parent_query: Query<&Parent>,
    q_select: Query<&CellMode, With<SelectedCell>>,
) {
    if let Ok(manual_marker) = cell.get(trigger.entity()) {
        for ancestor in parent_query.iter_ancestors(trigger.entity()) {
            if let Ok(cell_mode) = q_select.get(ancestor) {
                if *cell_mode != CellMode::Digit {
                    if !manual_marker.selected {
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
    mut move_ev: EventReader<MoveSelectCell>,
    mut commands: Commands,
    q_select: Single<(Entity, &CellPosition), With<SelectedCell>>,
    q_other: Query<(Entity, &CellPosition), Without<SelectedCell>>,
) {
    let (entity, cell_position) = q_select.into_inner();
    for ev in move_ev.read() {
        let new_position = match ev {
            MoveSelectCell::Up => {
                if cell_position.row() > 0 {
                    let new_position =
                        CellPosition::from_row_col(cell_position.row() - 1, cell_position.col());
                    Some(new_position)
                } else {
                    None
                }
            }
            MoveSelectCell::Down => {
                if cell_position.row() < 8 {
                    let new_position =
                        CellPosition::from_row_col(cell_position.row() + 1, cell_position.col());
                    Some(new_position)
                } else {
                    None
                }
            }
            MoveSelectCell::Left => {
                if cell_position.col() > 0 {
                    let new_position =
                        CellPosition::from_row_col(cell_position.row(), cell_position.col() - 1);

                    Some(new_position)
                } else {
                    None
                }
            }
            MoveSelectCell::Right => {
                if cell_position.col() < 8 {
                    let new_position =
                        CellPosition::from_row_col(cell_position.row(), cell_position.col() + 1);
                    Some(new_position)
                } else {
                    None
                }
            }
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
}
