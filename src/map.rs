use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_HEIGHT * SCREEN_WIDTH) as usize;

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    (y * SCREEN_WIDTH + x) as usize
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            return None;
        }
        Some(map_idx(point.x, point.y))
    }
    pub fn can_enter_tile(&self, point: Point) -> bool {
        let idx = self.try_idx(point);
        match idx {
            Some(x) => match self.tiles[x] {
                TileType::Floor => true,
                TileType::Wall => false,
            },
            None => false,
        }
    }
}
