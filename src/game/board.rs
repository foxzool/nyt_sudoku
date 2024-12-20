use crate::color::*;
use crate::game::cell_state::{AutoCandidates, CellMode, DigitValueCell, ManualCandidates};
use crate::game::position::CellPosition;
use crate::game::{
    AutoCandidateCellMarker, AutoCandidateMode, AutoCandidatesContainer, CellGrid, ConflictCount,
    DigitCellMarker, ManualCandidateCellMarker, ManualCandidatesContainer, MoveSelectCell,
    SelectedCell,
};
use crate::GameState;
use bevy::prelude::*;
use sudoku::board::Digit;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            show_digit_cell,
            show_manual_candidates,
            show_auto_candidates,
            show_preview_number,
        )
            .run_if(in_state(GameState::Playing)),
    )
    .add_systems(
        Update,
        switch_candidate_cell_mode.run_if(resource_changed::<AutoCandidateMode>),
    )
    .add_observer(move_select_cell);
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
    auto_mode: Res<AutoCandidateMode>,
) {
    for (entity, manual_candidates, cell_mode) in q_cell.iter() {
        if **auto_mode {
            continue;
        }
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
                }
            }
        }
    }
}

fn show_auto_candidates(
    q_cell: Query<(Entity, &AutoCandidates, &CellMode)>,
    children: Query<&Children>,
    mut candidate_cell: Query<(&mut TextColor, &mut AutoCandidateCellMarker)>,
    auto_mode: Res<AutoCandidateMode>,
) {
    for (entity, auto_candidates, cell_mode) in q_cell.iter() {
        if **auto_mode {
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
                    }
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


fn auto_candidate_cell_click(
    click: Trigger<Pointer<Click>>,
    mut cell: Query<&mut AutoCandidateCellMarker>,
    parent_query: Query<&Parent>,
    mut q_select: Query<&mut AutoCandidates, With<SelectedCell>>,
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

fn auto_candidate_cell_move(
    trigger: Trigger<Pointer<Over>>,
    mut cell: Query<&AutoCandidateCellMarker>,
    parent_query: Query<&Parent>,
    mut q_select: Query<&mut AutoCandidates, With<SelectedCell>>,
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

fn auto_candidate_cell_out(
    trigger: Trigger<Pointer<Out>>,
    mut cell: Query<&AutoCandidateCellMarker>,
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
    mut move_ev: Trigger<MoveSelectCell>,
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
    mut q_manual: Query<
        &mut Visibility,
        (
            With<ManualCandidatesContainer>,
            Without<AutoCandidatesContainer>,
        ),
    >,
    mut q_auto: Query<
        &mut Visibility,
        (
            With<AutoCandidatesContainer>,
            Without<ManualCandidatesContainer>,
        ),
    >,
    mut q_cell_mode: Query<&mut CellMode>
) {
    if **auto_mode {
        for mut visibility in q_manual.iter_mut() {
            *visibility = Visibility::Hidden;
        }
        for mut visibility in q_auto.iter_mut() {
            *visibility = Visibility::Visible;
        }
        for mut cell_mode in q_cell_mode.iter_mut() {
            if *cell_mode != CellMode::Digit {
                *cell_mode = CellMode::AutoCandidates;
            }
        }
    } else {
        for mut visibility in q_auto.iter_mut() {
            *visibility = Visibility::Hidden;
        }
        for mut visibility in q_manual.iter_mut() {
            *visibility = Visibility::Visible;
        }
        for mut cell_mode in q_cell_mode.iter_mut() {
            if *cell_mode != CellMode::Digit {
                *cell_mode = CellMode::ManualCandidates;
            }
        }
    }
}
