use rltk;
use rltk::{Rltk, RGB};

/// Tile type for map
#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

/// id for the tiles of map vector
pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

/// Makes a map with solid boundaries and randomly placed 400 walls.
/// Guarantee that it will look ugly
pub fn new_map_test() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80 * 50];
    // terminal boundaries = walls
    // horizontal walls
    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }
    // vertical walls
    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }
    // random walls everywhere
    let mut rng = rltk::RandomNumberGenerator::new();
    for _i in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);
        // player will be in middle so don't put wall there
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }
    map
}

/// Draws the map
pub fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    let mut y = 0;
    let mut x = 0;
    for tile in map.iter() {
        match tile {
            TileType::Floor => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.5, 0.5, 0.5),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('.'),
                );
            }
            TileType::Wall => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.0, 1.0, 0.0),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('#'),
                );
            }
        }
        // move the coords
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}
