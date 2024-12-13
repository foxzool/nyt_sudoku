use bevy::prelude::*;

use sudoku::board::CellState;

/// 格子的值
#[derive(Component, Debug)]
pub struct CellValue {
    current: CellState,
    previous: CellState,
}

impl CellValue {
    pub fn new(current: CellState) -> Self {
        Self {
            current,
            previous: current,
        }
    }

    pub fn set(&mut self, new: CellState) {
        self.previous = self.current;
        self.current = new;
    }

    pub fn current(&self) -> &CellState {
        &self.current
    }

    pub fn rollback(&mut self) {
        std::mem::swap(&mut self.current, &mut self.previous);
    }

    pub fn is_digit(&self) -> bool {
        matches!(self.current, CellState::Digit(_))
    }

    pub fn is_candidates(&self) -> bool {
        matches!(self.current, CellState::Candidates(_))
    }
}

/// 固定的格子， 不能修改
#[derive(Component)]
pub struct FixedCell;
