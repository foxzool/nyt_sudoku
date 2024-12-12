use crate::actions::Actions;
use crate::logic::position::CellPosition;
use crate::GameState;
use bevy::color::palettes::basic::{BLACK, GRAY};
use bevy::color::palettes::css;
use bevy::prelude::*;
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
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for SudokuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            (spawn_board, spawn_cells).chain(),
        )
        .add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
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
                for block_index in (0..9) {
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
                    },
                                   BackgroundColor(GRAY.into())
                    )).with_children(|builder| {
                        for bi in (0..9) {
                            let cell = block_index * 9 + bi;
                            builder.spawn((
                                CellPosition::new(cell),
                                Node {
                                    display: Display::Grid,
                                    // border: UiRect::all(Val::Px(2.)),
                                    ..default()
                                },
                                BorderColor(css::AQUA.into()),
                            )).with_children(|builder| {
                                builder.spawn((Node::default(), BackgroundColor(Color::WHITE)));
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

#[derive(Component)]
struct CellsLayout;

#[derive(Component)]
struct ControlLayout;

fn spawn_cells(
    mut commands: Commands,
    layout: Single<Entity, With<CellsLayout>>,
    asset_server: Res<AssetServer>,
) {
    let sudoku = Sudoku::generate();

    let solver = StrategySolver::from_sudoku(sudoku.clone());
    commands.insert_resource(SudokuManager {
        current_sudoku: sudoku,
    });

    for (index, cell_state) in solver.grid_state().into_iter().enumerate() {
        let cell_state = cell_state::CellState(cell_state);

        commands.entity(*layout).with_children(|commands| {
            commands
                .spawn((
                    cell_state,
                    CellPosition::new(index as u8),
                    Node {
                        display: Display::Grid,
                        // border: UiRect::all(Val::Px(1.)),
                        ..default()
                    },
                    Outline {
                        width: Val::Px(1.),
                        color: Color::srgb_u8(97, 97, 97),
                        ..default()
                    },
                    // BackgroundColor(BISQUE.into())
                ))
                .with_children(|builder| {
                    builder.spawn((
                        Node::default(),
                        BackgroundColor(bevy::color::palettes::css::BISQUE.into()),
                    ));
                });
        });
    }
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_secs(),
        actions.player_movement.unwrap().y * speed * time.delta_secs(),
        0.,
    );
    for mut player_transform in &mut player_query {
        player_transform.translation += movement;
    }
}
