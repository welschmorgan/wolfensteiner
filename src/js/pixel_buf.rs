use std::sync::{Arc, Mutex};

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct JSPixelBuffer {
    addr: usize,
    len: usize,
    width: usize,
    height: usize,
}

#[wasm_bindgen]
impl JSPixelBuffer {
    #[wasm_bindgen(constructor)]
    pub fn new(addr: usize, len: usize, w: usize, h: usize) -> Self {
        Self {
            addr,
            len,
            width: w,
            height: h,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn addr(&self) -> usize {
        self.addr
    }

    #[wasm_bindgen(getter)]
    pub fn len(&self) -> usize {
        self.len
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> usize {
        self.width
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> usize {
        self.height
    }
}


use lazy_static::lazy_static;

use crate::{PixelBuffer};

lazy_static! {
    pub static ref BUFFER: Arc<Mutex<PixelBuffer>> = Arc::new(Mutex::new(PixelBuffer {
        data: vec![],
        width: 0,
        height: 0
    }));
}

#[wasm_bindgen]
pub fn get_buffer() -> JSPixelBuffer {
    let g = BUFFER.lock().unwrap();
    let addr = g.data.as_ptr() as usize;
    JSPixelBuffer::new(addr, g.data.len(), g.width, g.height)
}
