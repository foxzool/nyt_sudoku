use bevy::prelude::*;
use std::ops::BitXorAssign;
use bevy::utils::HashSet;
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

/// 冲突红点
#[derive(Component, Default, Deref, DerefMut)]
pub struct ConflictCell(pub HashSet<Entity>);

#[derive(Component)]
pub struct RevealedCell;