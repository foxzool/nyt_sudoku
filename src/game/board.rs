use crate::color::*;
use crate::game::cell_state::{CellValue, FixedCell};
use crate::game::position::CellPosition;
use crate::game::{
    candidate_cell_click, AutoCandidateMode, CandidateCell, CandidatesContainer, CellGrid,
    ConflictCount, DigitCell, SelectedCell,
};
use crate::GameState;
use bevy::prelude::*;
use bevy::utils::info;
use std::time::Duration;
use sudoku::board::{CellState, Digit};

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        update_cell_number.run_if(in_state(GameState::Playing)),
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
                                                    DigitCell,
                                                ));

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
                                                                    TextColor(*DARK_BLACK),
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
                                                                    CandidateCell {
                                                                        index: i,
                                                                        auto_candidate_selected: false,
                                                                        manual_candidate_selected: false,
                                                                    },
                                                                ))
                                                                .observe(candidate_cell_move)
                                                                .observe(candidate_cell_out)
                                                                .observe(candidate_cell_click);
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
struct PreviewCandidate {
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

fn candidate_cell_move(
    trigger: Trigger<Pointer<Over>>,
    cell: Query<(Entity, &CandidateCell)>,
    parent_query: Query<&Parent>,
    q_select: Query<&SelectedCell>,
    auto_mode: Res<AutoCandidateMode>,
    mut commands: Commands,
) {
    let (entity, candidate_cell) = cell.get(trigger.entity()).unwrap();
    for ancestor in parent_query.iter_ancestors(entity) {
        if q_select.get(ancestor).is_ok() {
            if **auto_mode && !candidate_cell.auto_candidate_selected {
                commands
                    .entity(trigger.entity())
                    .insert(PreviewCandidate::hold());
            } else if !candidate_cell.manual_candidate_selected {
                commands
                    .entity(trigger.entity())
                    .insert(PreviewCandidate::hold());
            }
        }
    }
}

fn candidate_cell_out(
    trigger: Trigger<Pointer<Out>>,
    mut commands: Commands,
    parent_query: Query<&Parent>,
    q_select: Query<&SelectedCell>,
) {
    let mut find_selected = false;
    for ancestor in parent_query.iter_ancestors(trigger.entity()) {
        if q_select.get(ancestor).is_ok() {
            find_selected = true;
        }
    }

    if find_selected {
        commands
            .entity(trigger.entity())
            .insert(PreviewCandidate::default());
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

fn update_cell_number(
    cell: Query<(&CellValue, &Children, Option<&FixedCell>, &CellPosition)>,
    auto_mode: Res<AutoCandidateMode>,
    mut digit_cell: Query<
        (&mut Text, &mut Visibility),
        (With<DigitCell>, Without<CandidatesContainer>),
    >,
    mut candidates_container: Query<
        (&mut Visibility, &Children),
        (With<CandidatesContainer>, Without<DigitCell>),
    >,
    mut candidate_cell: Query<
        (
            &mut TextColor,
            &mut CandidateCell,
            Option<&mut PreviewCandidate>,
        ),
        (Without<DigitCell>, Without<CandidatesContainer>),
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
                                    } else {
                                        cell.manual_candidate_selected = true;
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
