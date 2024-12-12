use bevy::prelude::Component;

/// 数独格子的位置
#[derive(Component, Debug)]
pub struct CellPosition(pub u8);

/// 快速计算宫格索引
#[rustfmt::skip]
static BLOCK: [u8; 81] = [
    0, 0, 0, 1, 1, 1, 2, 2, 2,
    0, 0, 0, 1, 1, 1, 2, 2, 2,
    0, 0, 0, 1, 1, 1, 2, 2, 2,
    3, 3, 3, 4, 4, 4, 5, 5, 5,
    3, 3, 3, 4, 4, 4, 5, 5, 5,
    3, 3, 3, 4, 4, 4, 5, 5, 5,
    6, 6, 6, 7, 7, 7, 8, 8, 8,
    6, 6, 6, 7, 7, 7, 8, 8, 8,
    6, 6, 6, 7, 7, 7, 8, 8, 8,
];

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

    pub fn block(&self) -> u8 {
        BLOCK[self.0 as usize]
    }
}
