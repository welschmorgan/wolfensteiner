use std::{
    fmt::{Debug, Display},
    ops::Add,
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;

use crate::{console_log, Color, Rect};

use super::{PixelBuffer, Ray, ToPrimitive, Vec2};

#[derive(Debug, Clone, Copy, Default)]
pub struct Player {
    pub position: Vec2,
    pub direction: Vec2,
}

impl Player {
    pub fn new(position: Vec2, direction: Vec2) -> Self {
        Self {
            position,
            direction,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum WallKind {
    Basic,
}

#[derive(Clone, Copy, Debug)]
pub struct Wall {
    pub kind: WallKind,
    pub position: Vec2<i32>,
}

impl Wall {
    pub fn new(kind: WallKind, position: Vec2<i32>) -> Self {
        Self { kind, position }
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    pub player: Player,
    pub walls: Vec<Wall>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Player::new(Vec2::new(0f32, 0f32), Vec2::new(0f32, -1f32)),
            walls: vec![],
        }
    }

    pub fn with_player(mut self, p: Player) -> Self {
        self.player = p;
        self
    }

    pub fn with_player_position(mut self, pos: Vec2) -> Self {
        self.player.position = pos;
        self
    }

    pub fn with_player_direction(mut self, dir: Vec2) -> Self {
        self.player.direction = dir;
        self
    }

    pub fn with_wall(mut self, wall: Wall) -> Self {
        self.walls.push(wall);
        self
    }

    pub fn with_walls(mut self, walls: &[Wall]) -> Self {
        self.walls.extend(walls.to_vec());
        self
    }

    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn walls(&self) -> &Vec<Wall> {
        &self.walls
    }

    pub fn render(&self, dt: f32, buf: &mut PixelBuffer) {
        buf.fill(Color::black());
        // console_log!(
        //     "Rendering game, player_pos = {}, player_dir = {}",
        //     self.player.position,
        //     self.player.direction
        // );
        self.render_minimap(buf);
    }

    fn render_minimap(&self, buf: &mut PixelBuffer) {
        const MINIMAP_SIZE: Vec2<usize> = Vec2::new(100, 100);
        const MINIMAP_TILE_SIZE: Vec2<usize> = Vec2::new(5, 5);
        const MINIMAP_TILES_COUNT: Vec2<f32> = Vec2::new(
            (MINIMAP_SIZE.x as f32) / (MINIMAP_TILE_SIZE.x as f32),
            (MINIMAP_SIZE.y as f32) / (MINIMAP_TILE_SIZE.y as f32),
        );
        const MINIMAP_BORDER_COLOR: Option<Color> = Some(Color::blue());
        const MINIMAP_PLAYER_COLOR: Color = Color::red();
        if let Some(border_color) = MINIMAP_BORDER_COLOR.as_ref() {
            for y in 0..MINIMAP_SIZE.x {
                for x in 0..MINIMAP_SIZE.y {
                    if x == 0 || x == (MINIMAP_SIZE.x - 1) || y == 0 || y == (MINIMAP_SIZE.y - 1) {
                        let _ = buf.put(x, y, *border_color);
                    }
                }
            }
        }
        let world_bounds = Rect::new(
            self.player.position - MINIMAP_TILES_COUNT * MINIMAP_TILE_SIZE.cast::<f32>() * Vec2::scalar(0.5f32),
            self.player.position - MINIMAP_TILES_COUNT * MINIMAP_TILE_SIZE.cast::<f32>() * Vec2::scalar(0.5f32),
        );
        console_log!("minimap world bounds: {:?}")
        let player_pos = Vec2::new(
            (MINIMAP_SIZE.0 as f32 * 0.5f32) as usize,
            (MINIMAP_SIZE.0 as f32 * 0.5f32) as usize,
        );
        for y in player_pos.y - 1..player_pos.y + 1 {
            for x in player_pos.x - 1..player_pos.x + 1 {
                let _ = buf.put(x, y, MINIMAP_PLAYER_COLOR);
            }
        }
        buf.line(
            player_pos.cast::<isize>(),
            player_pos.cast::<isize>() + self.player.direction.scaled(20.0).cast::<isize>(),
            Color::green(),
        );
    }

    pub fn wall_at<T: Copy + ToPrimitive>(&self, pos: Vec2<T>) -> Option<&Wall> {
        self.walls.iter().find(|w| w.position == pos.cast::<i32>())
    }

    pub fn ascii_walls(&self) -> Vec<String> {
        let positions = self.walls.iter().map(|w| w.position).collect::<Vec<_>>();
        let bbox = (
            positions.iter().min().cloned().unwrap_or_default(),
            positions.iter().max().cloned().unwrap_or_default(),
        );
        let mut repr = vec![];
        for y in bbox.0.y..=bbox.1.y {
            let mut s = String::new();
            for x in bbox.0.x..=bbox.1.x {
                let cur_pos = Vec2::new(x, y);
                if self.player.position == cur_pos.cast::<f32>() {
                    s.push('p');
                } else {
                    s.push(match self.wall_at(cur_pos) {
                        Some(_) => 'x',
                        None => ' ',
                    })
                }
            }
            repr.push(s);
        }
        repr
    }

    pub fn shoot_ray<'a>(
        &'a self,
        start: Vec2<f32>,
        dir: Vec2<f32>,
    ) -> (Ray<f32>, Vec<(Vec2<f32>, &'a Wall)>) {
        let mut ray = Ray::new(start, dir, 100f32, 0.5).expect("failed to create ray");
        let intersections = ray
            .samples()
            .iter()
            .filter_map(|sample| self.wall_at(*sample).map(|w| (*sample, w)))
            .collect::<Vec<_>>();
        (ray, intersections)
    }
}

lazy_static! {
    pub static ref GAME: Arc<Mutex<Option<Game>>> = Arc::new(Mutex::new(None));
}
