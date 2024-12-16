use bevy::prelude::*;
use sudoku::bitset::Set;
use sudoku::board::CellState;

/// 格子的值
#[derive(Component, Debug)]
pub struct CellValue {
    current: CellState,
    candidates: CellState,
}

impl CellValue {
    pub fn new(current: CellState) -> Self {
        let candidates = match current {
            CellState::Digit(_) => CellState::Candidates(Set::NONE),
            CellState::Candidates(_) => current,
        };

        Self {
            current,
            candidates,
        }
    }

    pub fn set(&mut self, new: CellState) {
        if let CellState::Candidates(_) = new {
            self.candidates = new;
        }

        self.current = new;
    }

    pub fn current(&self) -> &CellState {
        &self.current
    }

    pub fn rollback(&mut self) {
        self.current = self.candidates;
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
