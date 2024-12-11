use bevy::prelude::Component;

/// 数独格子的位置
#[derive(Component, Debug)]
pub struct CellPosition(pub u8);


impl CellPosition {
    pub fn new(cell: u8) -> CellPosition {
        assert!(cell < 81);
        CellPosition(cell)
    }

    pub fn row(&self) -> u8 {
        self.0 / 9
    }

    pub fn col(&self) -> u8 {
        self.0 % 9
    }
}