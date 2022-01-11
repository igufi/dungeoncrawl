mod map;
mod map_builder;
mod camera;
mod components;
mod spawner;
mod systems;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::map::*;
    pub use crate::components::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;  // a multi-file module folder

}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,  // no longer needed with Legion ecs?
    systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs, map_builder.player_start);   // we will create the player only once
        resources.insert(map_builder.map);      // store the map that map_builder makes as resource
        resources.insert(Camera::new(map_builder.player_start));

        Self { 
            ecs,
            resources,  // no longer needed with Legion ecs?
            systems: build_scheduler()  //TODO
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.resources.insert(ctx.key);  // Stores keyboard state
        self.systems.execute(&mut self.ecs, &mut self.resources);  // &mut borrows are exclusive
        // TODO - Render Draw Buffer
    }
}


// continue from page 110 (123 on pdf)

fn main() -> BError {
    let context = BTermBuilder::new()       // creastes new, *uncofigured* session
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        // we will create two layers, one for map and one for player:
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;

    main_loop(context, State::new())
    
}
