use crate::game::position::CellPosition;
use crate::game::{
    candidate_cell_click, candidate_cell_move, candidate_cell_out, on_click_cell, CandidateCell,
    CandidatesContainer, CellGrid, DigitCell,
};
use bevy::color::palettes::basic::GRAY;
use bevy::prelude::*;

pub(crate) fn main_board(font: &Handle<Font>, builder: &mut ChildBuilder) {
    builder
        .spawn((
            Node {
                margin: UiRect::axes(Val::Px(24.0), Val::Px(0.)),
                ..default()
            }, BackgroundColor(GRAY.into()),
        )).with_children(|builder| {
        // 生成9宫格布局
        builder
            .spawn((
                Node {
                    height: Val::Percent(100.0),
                    aspect_ratio: Some(1.0),
                    display: Display::Grid,
                    grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
                    grid_template_rows: RepeatedGridTrack::flex(3, 1.0),
                    row_gap: Val::Px(4.0),
                    column_gap: Val::Px(4.0),
                    border: UiRect::all(Val::Px(4.0)),
                    ..default()
                },
                // BorderColor(Color::BLACK),
                // BackgroundColor(Color::WHITE),
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
                            BackgroundColor(GRAY.into()),
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
                                                        font: font.clone(),
                                                        font_size: 46.0,
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

                                                // 候选格子容器
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
                                                        CandidatesContainer,
                                                    ))
                                                    .with_children(|builder| {
                                                        // 9个候选数字格子
                                                        for i in 1..=9u8 {
                                                            builder
                                                                .spawn((
                                                                    Text::new(i.to_string()),
                                                                    TextFont {
                                                                        font: font.clone(),
                                                                        font_size: 14.0,
                                                                        ..default()
                                                                    },
                                                                    TextColor(Color::srgba_u8(
                                                                        18, 18, 18, 0,
                                                                    )),
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
                                                                        selected: false,
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
