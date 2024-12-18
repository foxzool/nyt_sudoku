use bevy::prelude::*;
use sudoku::bitset::Set;
use sudoku::board::CellState;

/// 格子的值
#[derive(Component, Debug)]
pub struct CellValue {
    current: CellState,
    auto_candidates: CellState,
    manual_candidates: CellState,
}

impl CellValue {
    pub fn new(current: CellState) -> Self {
        let candidates = match current {
            CellState::Digit(_) => CellState::Candidates(Set::NONE),
            CellState::Candidates(_) => current,
        };

        Self {
            current,
            auto_candidates: candidates,
            manual_candidates: CellState::Candidates(Set::NONE),
        }
    }

    pub fn set(&mut self, new: CellState, auto_mode: bool) {
        if let CellState::Candidates(_) = new {
            if auto_mode {
                self.auto_candidates = new;
            } else {
                self.manual_candidates = new;
            }
        }

        self.current = new;
    }

    pub fn current(&self, auto_mode: bool) -> &CellState {
        match self.current {
            CellState::Digit(_) => &self.current,
            CellState::Candidates(_) => {
                if auto_mode {
                    &self.auto_candidates
                } else {
                    &self.manual_candidates
                }
            }
        }
    }

    pub fn rollback(&mut self, auto_mode: bool) {
        match self.current {
            CellState::Digit(_) => {
                if auto_mode {
                    self.current = self.auto_candidates;
                } else {
                    self.current = self.manual_candidates;
                }
            }
            CellState::Candidates(_) => {
                if !auto_mode {
                    self.manual_candidates = CellState::Candidates(Set::NONE);
                    self.current = self.manual_candidates;
                }
            }
        }
    }
}

/// 固定的格子， 不能修改
#[derive(Component)]
pub struct FixedCell;
