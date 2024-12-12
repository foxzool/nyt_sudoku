use bevy::prelude::*;

use sudoku::board::CellState;

/// 格子的值
#[derive(Component, Debug, Deref, DerefMut)]
pub struct CellValue(pub CellState);

/// 固定的格子， 不能修改
#[derive(Component)]
pub struct FixedCell;
