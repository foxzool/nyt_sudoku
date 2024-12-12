use bevy::prelude::*;

use sudoku::board::CellState;

/// 格子的值
#[derive(Component, Debug, Deref, DerefMut)]
pub struct CellValue(pub CellState);

#[derive(Component)]
pub struct FixedCell;
