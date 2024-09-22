use wasm_bindgen::prelude::wasm_bindgen;

use crate::{console_log, Color, Vec2, Wall, WallKind, BUFFER};
use crate::{Game, GAME};

#[wasm_bindgen]
pub fn init_game(w: usize, h: usize) {
    let mut g = GAME.lock().unwrap();
    let mut walls = vec![];
    for y in -2..=2 {
        for x in -2..=2 {
            if x == -2 || x == 2 || y == -2 || y == 2 {
                walls.push(Wall::new(WallKind::Basic, Vec2::new(x, y)))
            }
        }
    }
    *g = Some(Game::new().with_walls(&walls));
    console_log!("Walls: {:#?}", g.as_ref().unwrap().ascii_walls());
    let mut buf = BUFFER.lock().unwrap();
    buf.data.clear();
    buf.data.extend(vec![
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255
        };
        w * h * 4
    ]);
    buf.width = w;
    buf.height = h;
}

#[wasm_bindgen]
pub fn render_game(dt: f32) {
    let mut game = GAME.lock().unwrap();
    let game = game.as_mut().unwrap();
    let mut buf = BUFFER.lock().unwrap();
    game.render(dt, &mut buf);
    let (ray, intersections) = game.shoot_ray(game.player.position, game.player.direction);
    // for (pos, wall) in intersections {
    //     console_log!("ray intersected with {:?} wall at {}", wall.kind, pos);
    // }
    // game.player.direction.rotate(1.0f32);
}

#[wasm_bindgen]
pub fn move_mouse(x: i32, _: i32) {
    let mut g = GAME.lock().unwrap();
    let g = g.as_mut().unwrap();
    g.player.direction.rotate(x as f32);
    g.player.direction.normalize();
}
