use std::{sync::Mutex};

use crate::console_log;

use super::{Error, Vec2};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn transparent() -> Self {
        Self::new(0, 0, 0, 0)
    }

    pub const fn black() -> Self {
        Self::new(0, 0, 0, 255)
    }

    pub const fn white() -> Self {
        Self::new(255, 255, 255, 255)
    }

    pub const fn red() -> Self {
        Self::new(255, 0, 0, 255)
    }

    pub const fn green() -> Self {
        Self::new(0, 255, 0, 255)
    }

    pub const fn blue() -> Self {
        Self::new(0, 0, 255, 255)
    }
}

pub struct PixelBuffer {
    pub data: Vec<Color>,
    pub width: usize,
    pub height: usize,
}

impl PixelBuffer {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            data: vec![Color::new(0, 0, 0, 255); w * h],
            width: w,
            height: h,
        }
    }

    pub fn addr(&self, x: usize, y: usize) -> Option<usize> {
        let addr = y * self.width + x;
        if addr < self.data.len() {
            return Some(addr);
        }
        None
    }

    pub fn put(&mut self, x: usize, y: usize, c: Color) -> crate::Result<()> {
        if x >= self.width || y >= self.height {
            return Err(Error::OutOfBounds2D {
                point: (x as f32, y as f32),
                range: (0f32..self.width as f32, 0f32..self.height as f32),
            });
        }
        self.data[y * self.width + x] = c;
        Ok(())
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Color> {
        self.addr(x, y).and_then(|addr| self.data.get(addr))
    }

    pub fn line(&mut self, start: Vec2<isize>, end: Vec2<isize>, color: Color) {
        let mut cur = start.cast::<f32>();
        // let mut step_id = 0;
        // let limit = 10;
        if start == end {
            return;
        }
        // while step_id < limit {
        loop {
            let raw_dir = (end.cast::<f32>() - cur).cast::<isize>();
            let mut dir: Vec2<f32> = raw_dir.cast::<f32>();
            let goal = dir.normalize();
            // console_log!("line({}, {}):step #{} | cur = {}, raw_dir = {}, dir = {}, goal = {}", start, end, step_id, cur, raw_dir, dir, goal);
            if goal <= 1f32 {
                break;
            }
            let _ = self.put(cur.x as usize, cur.y as usize, color);
            cur = cur + dir;
            // step_id += 1;
        }
    }

    pub fn fill(&mut self, c: Color) {
        for i in 0..self.data.len() {
            self.data[i] = c;
        }
    }
}
