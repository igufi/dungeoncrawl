use crate::prelude::*;
// usize = preferred bit size for the CPU
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;


/*
- Clone allows to work around the borrow-checker
- Copy changes the action when assigning type from
one variable to another - in this context it copies 
the value instead of moving it.
- PartialEq allows us to use == with TileType values
*/
#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES], // creates 4000 Floor-tiles in an array
        }
    }
    


    // Helper to check if a given point is within screen limits
    pub fn in_bounds(&self, point : Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH
        && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    // Helper to check if a given point can be entered by creature
    pub fn can_enter_title(&self, point : Point) -> bool {
        self.in_bounds(point)
            && self.tiles[map_idx(point.x, point.y)]==TileType::Floor
    }

    // Helper that returns Option with nothing if we are out of bounds,
    // otherwise we get the absolute value of the map index vec.
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }

}

// Map indexing, returns the absolute location in the vec
// Index is Y-first, each row together
// result is usize because vectors are indexed by a var of type usize
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}
