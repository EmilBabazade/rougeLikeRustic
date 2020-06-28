mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
use player::*;
mod rect;
pub use rect::*;

use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;

pub struct State {
    pub ecs: World,
}

impl State {
    pub fn new() -> Self {
        State { ecs: World::new() }
    }

    // fn run_systems(&mut self) {
    //     unimplemented!();
    // }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        player_input(self, ctx);
        // self.run_systems();
        // draw the map BEFORE drawing other stuff
        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);
        // draw player, enemies, etc.
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(
                pos.x,
                pos.y,
                render.foreground,
                render.background,
                render.glyph,
            );
        }
    }
}

fn main() -> rltk::BError {
    // terminal to draw in
    let context = rltk::RltkBuilder::simple80x50()
        .with_title("Rougelike Tutorial")
        .build()?;
    // register the components
    let mut game_state = State::new();
    game_state.ecs.register::<Position>();
    game_state.ecs.register::<Renderable>();
    game_state.ecs.register::<Player>();
    // insert the map as resource
    game_state.ecs.insert(new_map_test());
    // create some entities to be drawn on the terminal
    game_state
        .ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            foreground: RGB::named(rltk::YELLOW),
            background: RGB::named(rltk::BLACK),
        })
        .with(Player)
        .build();
    // start the game
    rltk::main_loop(context, game_state)
}
