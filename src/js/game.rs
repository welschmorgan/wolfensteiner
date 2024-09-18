use wasm_bindgen::prelude::wasm_bindgen;

use crate::{Color, BUFFER};
use crate::{Game, GAME};

#[wasm_bindgen]
pub fn init_game(w: usize, h: usize) {
    *GAME.lock().unwrap() = Some(Game::new());
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
pub fn render_game() {
    let mut game = GAME.lock().unwrap();
    let mut buf = BUFFER.lock().unwrap();
    game.as_ref().unwrap().render(&buf);
}
