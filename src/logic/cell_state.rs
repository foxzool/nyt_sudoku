use bevy::prelude::Component;

use sudoku::board::CellState as CellStateEnum;

#[derive(Component, Debug)]
pub struct CellState(pub CellStateEnum);


