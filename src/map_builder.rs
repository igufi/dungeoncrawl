use crate::prelude::*;
const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map : Map,
    pub rooms : Vec<Rect>,
    pub player_start : Point,
}

impl MapBuilder {
    fn fill(&mut self, tile : TileType) {
        // out of the 4000 tiles, iterate through each and
        // the *T means to change the actual value of t and not the
        // reference ("de-referencing").
        self.map.tiles.iter_mut().for_each(|t| *t = tile);

    }

    fn build_random_rooms(&mut self, rng : &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true; // bad room, rest is skipped and we try again
                }
            }
            if !overlap {
                room.for_each(|p| {     // bracket function, differs from normal closure!
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0
                    && p.y < SCREEN_HEIGHT
                    {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });

                self.rooms.push(room)  // good room, no overlap and inside screen limits
            }
        }
    }
}