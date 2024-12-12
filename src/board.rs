use crate::board::cell_state::{CellValue, FixedCell};
use crate::board::position::CellPosition;
use crate::GameState;
use bevy::color::palettes::basic::{BLACK, GRAY};
use bevy::prelude::*;
use sudoku::board::CellState;
use sudoku::strategy::StrategySolver;
use sudoku::Sudoku;

mod cell_state;
mod position;

pub struct SudokuPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Resource, Debug)]
pub struct SudokuManager {
    pub current_sudoku: Sudoku,
}

/// This plugin handles player related stuff like movement
/// Player board is only active during the State `GameState::Playing`
impl Plugin for SudokuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            (spawn_board, init_cells).chain(),
        )
        .add_systems(Update, (update_cell).run_if(in_state(GameState::Playing)))
        .add_observer(on_select_cell)
        .add_observer(on_unselect_cell);
    }
}

fn spawn_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/franklin-normal-600.ttf");
    commands.spawn((
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
                GridTrack::flex(1.0)
            ],
            ..default()
        },
        BackgroundColor(Color::WHITE),
    )).with_children(|builder| {
        // 顶部菜单栏
        builder
            .spawn(
                Node {
                    display: Display::Grid,
                    // Make this node span two grid columns so that it takes up the entire top tow
                    grid_column: GridPlacement::span(2),
                    padding: UiRect::all(Val::Px(6.0)),
                    ..default()
                },
            )
            .with_children(|builder| {
                builder.spawn((
                    Text::new("top-level grid"),
                    TextFont { font: font.clone(), ..default() },
                    TextColor::BLACK,
                ));
            });

        // 格子布局容器
        builder
            .spawn((
                Node {
                    margin: UiRect::axes(Val::Px(24.0), Val::Px(0.)),
                    ..default()
                }, BackgroundColor(GRAY.into()),
            )).with_children(|builder| {
            // 生成9宫格布局
            builder.spawn((
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
            )).with_children(|builder| {
                // 生成九个宫格
                for block_index in 0..9 {
                    builder.spawn((Node {
                        height: Val::Percent(100.0),
                        aspect_ratio: Some(1.0),
                        display: Display::Grid,
                        grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
                        grid_template_rows: RepeatedGridTrack::flex(3, 1.0),
                        row_gap: Val::Px(1.0),
                        column_gap: Val::Px(1.0),
                        // border: UiRect::all(Val::Px(1.)),
                        ..default()
                    }, BackgroundColor(GRAY.into())
                    )).with_children(|builder| {
                        // 生成宫格里的9个格子
                        for bi in 0..9 {
                            let cell = block_index * 9 + bi;
                            builder.spawn((
                                Node {
                                    display: Display::Grid,
                                    ..default()
                                },
                            )).with_children(|builder| {
                                // 格子
                                builder.spawn((
                                    CellPosition::new(cell),
                                    Node {
                                        align_items: AlignItems::Center,
                                        justify_items: JustifyItems::Center,
                                        align_content: AlignContent::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                }, BackgroundColor(Color::WHITE), CellGrid))
                                    .observe(on_click_cell)
                                    .with_children(|builder| {
                                        builder.spawn((
                                            Text::new(cell.to_string()),
                                            TextFont { font: font.clone(),
                                                font_size: 42.0,
                                            ..default() },
                                            TextColor(Color::srgb_u8(18,18,18)),
                                            Visibility::Hidden,
                                            Node {
                                                margin: UiRect {
                                                  bottom: Val::Px(1.0),
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            DigitCell
                                        ));
                                    });
                                // builder.spawn((Node::default(), BackgroundColor(Color::WHITE)));
                            });
                        }
                    });
                }
            });
        });


        // 右侧边栏
        builder
            .spawn((
                Node {
                    display: Display::Grid,
                    // Align content towards the start (top) in the vertical axis
                    align_items: AlignItems::Start,
                    // Align content towards the center in the horizontal axis
                    justify_items: JustifyItems::Center,
                    // Add 10px padding
                    padding: UiRect::all(Val::Px(10.)),
                    // Add an fr track to take up all the available space at the bottom of the column so that the text nodes
                    // can be top-aligned. Normally you'd use flexbox for this, but this is the CSS Grid example so we're using grid.
                    grid_template_rows: vec![GridTrack::auto(), GridTrack::auto(), GridTrack::fr(1.0)],
                    // Add a 10px gap between rows
                    row_gap: Val::Px(10.),
                    ..default()
                },
                BackgroundColor(BLACK.into()),
            ))
            .with_children(|builder| {
                builder.spawn((Text::new("Sidebar"),
                               TextFont {
                                   font: font.clone(),
                                   ..default()
                               },
                ));
                builder.spawn((Text::new("A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely."),
                               TextFont {
                                   font: font.clone(),
                                   font_size: 13.0,
                                   ..default()
                               },
                ));
                builder.spawn(Node::default());
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

#[derive(Component)]
struct ControlLayout;

fn init_cells(mut commands: Commands, cell_background: Query<(Entity, &CellPosition)>) {
    let sudoku = Sudoku::generate();
    info!("sudoku: {:?}", sudoku);

    let solver = StrategySolver::from_sudoku(sudoku.clone());
    commands.insert_resource(SudokuManager {
        current_sudoku: sudoku,
    });

    'l: for (index, cell_state) in solver.grid_state().into_iter().enumerate() {
        let cell_value = CellValue(cell_state);

        for (entity, cell_position) in cell_background.iter() {
            if cell_position.0 == index as u8 {
                match &cell_value.0 {
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
    cell: Query<(&CellValue, &Children, Option<&FixedCell>), Changed<CellValue>>,
    mut cell_digit: Query<(&mut Text, &mut Visibility), With<DigitCell>>,
) {
    for (cell_value, children, opt_fixed) in cell.iter() {
        match cell_value.0 {
            CellState::Digit(digit) => {
                for child in children.iter() {
                    if let Ok((mut text, mut visibility)) = cell_digit.get_mut(*child) {
                        text.0 = digit.get().to_string();
                        visibility.toggle_visible_hidden();
                    }
                }
            }
            CellState::Candidates(_) => {}
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
