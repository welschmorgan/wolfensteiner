use std::{
    fmt::{Debug, Display},
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;

use crate::{console_log};

use super::PixelBuffer;

#[derive(Clone, Copy, Default)]
pub struct Vec2<T: Copy = f32> {
    pub x: T,
    pub y: T,
}

impl<T: Copy + Debug> Debug for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vec2")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

impl<T: Copy + Display> Display for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: Copy> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[derive(Default)]
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

pub enum WallKind {}

pub struct Wall {
    pub kind: WallKind,
    pub position: Vec2<i32>,
}

impl Wall {
    pub fn new(kind: WallKind, position: Vec2<i32>) -> Self {
        Self { kind, position }
    }
}

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

    pub fn with_walls(mut self, walls: Vec<Wall>) -> Self {
        self.walls.extend(walls);
        self
    }

    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn walls(&self) -> &Vec<Wall> {
        &self.walls
    }

    pub fn render(&self, buf: &PixelBuffer) {
        console_log!(
            "Rendering game, player_pos = {}, player_dir = {}",
            self.player.position,
            self.player.direction
        )
    }
}

lazy_static! {
    pub static ref GAME: Arc<Mutex<Option<Game>>> = Arc::new(Mutex::new(None));
}
