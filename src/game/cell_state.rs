use bevy::prelude::*;
use std::ops::BitOrAssign;
use sudoku::bitset::Set;
use sudoku::board::{CellState, Digit};

/// 格子的值
#[derive(Component, Debug)]
pub struct CellValue {
    current: CellState,
    digit: Option<Digit>,
    auto_candidates: Set<Digit>,
    manual_candidates: Set<Digit>,
}

impl CellValue {
    pub fn new(current: CellState) -> Self {
        match current {
            CellState::Digit(digit) => CellValue {
                current,
                digit: Some(digit),
                auto_candidates: Set::NONE,
                manual_candidates: Set::NONE,
            },
            CellState::Candidates(digit_set) => CellValue {
                current,
                digit: None,
                auto_candidates: digit_set,
                manual_candidates: Set::NONE,
            },
        }
    }

    pub fn add_value(&mut self, new: CellState, auto_mode: bool) {
        match new {
            CellState::Digit(digit) => {
                self.digit = Some(digit);
                self.current = new;
            }
            CellState::Candidates(new_digit_set) => {
                if auto_mode {
                    self.auto_candidates.bitor_assign(new_digit_set);
                    self.current = CellState::Candidates(self.auto_candidates);
                } else {
                    self.manual_candidates.bitor_assign(new_digit_set);
                    self.current = CellState::Candidates(self.manual_candidates);
                }
            }
        }
    }

    pub fn set(&mut self, new: CellState, auto_mode: bool) {
        match new {
            CellState::Digit(digit) => {
                self.digit = Some(digit);
            }
            CellState::Candidates(digit_set) => {
                if auto_mode {
                    self.auto_candidates = digit_set;
                } else {
                    self.manual_candidates = digit_set;
                }
            }
        }

        self.current = new;
    }

    pub fn current(&self, auto_mode: bool) -> CellState {
        match self.current {
            CellState::Digit(_) => self.current,
            CellState::Candidates(_) => {
                if auto_mode {
                    CellState::Candidates(self.auto_candidates)
                } else {
                    CellState::Candidates(self.manual_candidates)
                }
            }
        }
    }

    pub fn rollback(&mut self, auto_mode: bool) -> Option<Digit> {
        match self.current {
            CellState::Digit(digit) => {
                if auto_mode {
                    self.current = CellState::Candidates(self.auto_candidates);
                } else {
                    self.current = CellState::Candidates(self.manual_candidates);
                }
                Some(digit)
            }
            CellState::Candidates(_) => {
                if !auto_mode {
                    self.manual_candidates = Set::NONE;
                    self.current = CellState::Candidates(self.manual_candidates);
                }
                None
            }
        }
    }
}

/// 固定的格子， 不能修改
#[derive(Component)]
pub struct FixedCell;
