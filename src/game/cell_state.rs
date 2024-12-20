use bevy::prelude::*;
use std::ops::{BitOrAssign, BitXorAssign, Deref};
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
    pub fn from_cell_state(cell_state: CellState, auto_candidates: bool) -> Self {
        let cell_mode = if auto_candidates {
            CellMode::AutoCandidates
        } else {
            CellMode::ManualCandidates
        };
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
                cell_mode,
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
impl CandidatesValue for AutoCandidates {
    fn candidates_mut(&mut self) -> &mut Set<Digit> {
        &mut self.0
    }
    fn candidates(&self) -> &Set<Digit> {
        &self.0
    }
}

#[derive(Component, Debug)]
pub struct ManualCandidates(pub Set<Digit>);

impl CandidatesValue for ManualCandidates {
    fn candidates_mut(&mut self) -> &mut Set<Digit> {
        &mut self.0
    }

    fn candidates(&self) -> &Set<Digit> {
        &self.0
    }
}

pub trait CandidatesValue: Component {
    fn insert(&mut self, digit: Digit) {
        self.candidates_mut().bitxor_assign(digit);
    }

    fn candidates_mut(&mut self) -> &mut Set<Digit>;
    fn candidates(&self) -> &Set<Digit>;
}

#[derive(Component, Debug, PartialEq, Eq)]
pub enum CellMode {
    Digit,
    AutoCandidates,
    ManualCandidates,
}

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


/// 手动候选数字
#[derive(Component, Debug)]
pub struct ManualCandidateCellMarker {
    pub index: u8,
    pub selected: bool,
}

impl CandidateMarker for ManualCandidateCellMarker {
    fn index(&self) -> u8 {
        self.index
    }

    fn selected(&self) -> bool {
        self.selected
    }

    fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
}

/// 自动候选数字
#[derive(Component, Debug)]
pub struct AutoCandidateCellMarker {
    pub index: u8,
    pub selected: bool,
}

impl CandidateMarker for AutoCandidateCellMarker {
    fn index(&self) -> u8 {
        self.index
    }

    fn selected(&self) -> bool {
        self.selected
    }

    fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
}

pub trait CandidateMarker: Component {
    fn index(&self) -> u8;
    fn selected(&self) -> bool;

    fn set_selected(&mut self, selected: bool);
}