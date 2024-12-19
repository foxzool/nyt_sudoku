use bevy::prelude::*;
use std::ops::{BitOrAssign, BitXorAssign};
use sudoku::bitset::Set;
use sudoku::board::{CellState, Digit};

#[derive(Bundle)]
pub struct CellValueBundle {
    pub digit_value: DigitValueCell,
    pub auto_candidates: AutoCandidates,
    pub manual_candidates: ManualCandidates,
    pub cell_mode: CellMode,
}

impl CellValueBundle {
    pub fn from_cell_state(cell_state: CellState) -> Self {
        let (digit_value, auto_candidates, manual_candidates, cell_mode) = match cell_state {
            CellState::Digit(digit) => (
                DigitValueCell(Some(digit)),
                AutoCandidates(Set::NONE),
                ManualCandidates(Set::NONE),
                CellMode::Digit,
            ),
            CellState::Candidates(digit_set) => (
                DigitValueCell(None),
                AutoCandidates(digit_set),
                ManualCandidates(Set::NONE),
                CellMode::ManualCandidates,
            ),
        };

        CellValueBundle {
            digit_value,
            auto_candidates,
            manual_candidates,
            cell_mode,
        }
    }
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct DigitValueCell(pub Option<Digit>);

#[derive(Component, Debug)]
pub struct AutoCandidates(pub Set<Digit>);

impl AutoCandidates {
    pub fn insert(&mut self, digit: Digit) {
        self.0.bitxor_assign(digit);
    }
}

#[derive(Component, Debug)]
pub struct ManualCandidates(pub Set<Digit>);

impl ManualCandidates {
    pub fn insert(&mut self, digit: Digit) {
        self.0.bitxor_assign(digit);
    }

}

#[derive(Component, Debug, PartialEq, Eq)]
pub enum CellMode {
    Digit,
    AutoCandidates,
    ManualCandidates,
}

#[derive(Component, Debug)]
pub struct CellValueNew(pub CellState);

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

    pub fn insert(&mut self, new: CellState, auto_mode: bool) {
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
