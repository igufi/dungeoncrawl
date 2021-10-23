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
    
    // Render by row
    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y); // find the absolute location on vec

                // which enum TileType is the tile under index in question?
                match self.tiles[idx] {
                    TileType::Floor => {
                        ctx.set(x, y, YELLOW, BLACK, 
                            to_cp437('.')
                        );
                    }
                    TileType::Wall => {
                        ctx.set(x, y, GREEN, BLACK,
                        to_cp437('#')
                    );
                    }
                }
            }
        }
    }
}

// Map indexing, returns the absolute location in the vec
// Index is Y-first, each row together
// result is usize because vectors are indexed by a var of type usize
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}
