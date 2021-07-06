use ggez::{
    event::{Axis, Button, KeyCode},
    graphics::{DrawParam, Rect},
    mint::{Point2, Vector2},
    nalgebra as na,
};
use rand::{self, Rng};

/// The size of out game board in terms of how many grid
/// cells it takes up.
pub const GRID_SIZE: (i16, i16) = (30, 20);
/// The pixel size of each tile.
pub const GRID_CELL_SIZE: (i16, i16) = (32, 32);

/// The size of the window.
pub const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

/// The size of a sprite.
pub const SPRITE_SIZE: (i16, i16) = (32, 32);
/// The ratio of the cell size to sprite size
pub const SPRITE_CELL_RATIO: (f32, f32) = (
    (GRID_CELL_SIZE.0 as f32) / (SPRITE_SIZE.0 as f32),
    (GRID_CELL_SIZE.1 as f32) / (SPRITE_SIZE.1 as f32),
);

/// The number of updates we want to run each second.
pub const UPDATES_PER_SECOND: f32 = 8.0;
/// The number of milliseconds per each update.
pub const MILLIS_PER_UPDATE: u64 = (1000.0 / UPDATES_PER_SECOND) as u64;

/// This trait provides an "arithmetic" modulo function,
/// which works well for wrapping negative values.
pub trait ModuloSigned {
    fn modulo_signed(&self, n: Self) -> Self;
}

impl<T> ModuloSigned for T
where
    T: std::ops::Add<Output = T> + std::ops::Rem<Output = T> + Clone,
{
    fn modulo_signed(&self, n: Self) -> Self {
        (self.clone() % n.clone() + n.clone()) % n.clone()
    }
}

/// Contains all relevent errors
/// for our game.
#[derive(Debug, Clone)]
pub enum SnakeError {
    LogicError(String),
}
pub type SnakeResult<T = ()> = Result<T, SnakeError>;

/// Represents all the possible directions
/// that our snake can move.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    /// Allows us to easily get the inverse of a `Direction`.
    ///
    /// This allows us to easily check if the can move in
    /// a given direction.
    pub fn inverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    /// Converts between a `ggez` `KeyCode` and the `Direction` it represents.
    ///
    /// Not every `KeyCode` represents a `Direction`, so `None` is returned
    /// if this is the case.
    pub fn from_keycode(keycode: KeyCode) -> Option<Self> {
        match keycode {
            KeyCode::Up => Some(Direction::Up),
            KeyCode::Down => Some(Direction::Down),
            KeyCode::Left => Some(Direction::Left),
            KeyCode::Right => Some(Direction::Right),
            _ => None,
        }
    }

    /// Converts between a `ggez` `Button` and the `Direction` it represents.
    ///
    /// Not every `Button` represents a `Direction`, so `None` is returned
    /// if this is the case.
    pub fn from_button(button: Button) -> Option<Self> {
        match button {
            Button::DPadUp => Some(Direction::Up),
            Button::DPadDown => Some(Direction::Down),
            Button::DPadLeft => Some(Direction::Left),
            Button::DPadRight => Some(Direction::Right),
            _ => None,
        }
    }

    /// Converts between a `ggez` `Axis` and value (`f32`) to the direction they represent.
    ///
    /// Not every `Axis` represents a `Direction` in our case,   
    /// so `None` is returned if the is the case.
    ///
    /// Note that we also have a deadzone in our axis to prevent over-sensitive behavior.
    pub fn from_axis(axis: Axis, value: f32) -> Option<Self> {
        const CUTOFF: f32 = 0.4;
        match axis {
            Axis::RightStickX => {
                if value > CUTOFF {
                    Some(Direction::Right)
                } else if value < -CUTOFF {
                    Some(Direction::Left)
                } else {
                    None
                }
            }
            Axis::RightStickY => {
                if value > CUTOFF {
                    Some(Direction::Up)
                } else if value < -CUTOFF {
                    Some(Direction::Down)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl From<Direction> for Vector2<f32> {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Left => [-1.0, 0.0].into(),
            Direction::Up => [0.0, -1.0].into(),
            Direction::Right => [1.0, 0.0].into(),
            Direction::Down => [0.0, 1.0].into(),
        }
    }
}

/// Represents the possible things that the
/// snake could have eaten each update.
///
/// It could have either eaten a piece of `Food` or
/// itself if its head ran into its body.
#[derive(Clone, Copy)]
pub enum Ate {
    Itself,
    Food,
}

/// Represents a location on the grid / game board.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GridPosition {
    pub x: i16,
    pub y: i16,
}

impl GridPosition {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    /// Creates a random `GridPosition` from `(0, 0)` to `(max_x, max_y)`.
    pub fn random(max_x: i16, max_y: i16) -> Self {
        let mut rng = rand::thread_rng();
        (
            rng.gen_range::<i16, i16, i16>(0, max_x),
            rng.gen_range::<i16, i16, i16>(0, max_y),
        )
            .into()
    }

    /// Created a `GridPosition` from another position and a `Direction`.
    pub fn new_from_move(position: GridPosition, direction: Direction) -> Self {
        match direction {
            Direction::Up => {
                GridPosition::new(position.x, (position.y - 1).modulo_signed(GRID_SIZE.1))
            }
            Direction::Down => {
                GridPosition::new(position.x, (position.y + 1).modulo_signed(GRID_SIZE.1))
            }
            Direction::Left => {
                GridPosition::new((position.x - 1).modulo_signed(GRID_SIZE.0), position.y)
            }
            Direction::Right => {
                GridPosition::new((position.x + 1).modulo_signed(GRID_SIZE.0), position.y)
            }
        }
    }
}

impl From<(i16, i16)> for GridPosition {
    fn from(pos: (i16, i16)) -> Self {
        Self { x: pos.0, y: pos.1 }
    }
}

/// Helper implementation to eaily convert our `GridPosition`
/// to screen coordinates.
impl From<GridPosition> for Point2<f32> {
    fn from(pos: GridPosition) -> Self {
        Self {
            x: (pos.x * GRID_CELL_SIZE.0) as f32,
            y: (pos.y * GRID_CELL_SIZE.1) as f32,
        }
    }
}

/// The various sprites available in the spritesheet.
///
/// Use in conjuction with `Game::get_param` to easily get
/// the correct `DrawParam` that draws the asked for sprite.
pub enum Sprite {
    /// The head sprites.
    ///
    /// `Direction` is the direction in which the head is pointing.
    Head(Direction),
    /// The segment sprites. This include both the straight and curved
    /// body segments.
    ///
    /// The first and second `Direction`s segment starts and ends, respectivly.
    ///
    /// If the same `Direction` is passed twice, `Game::get_param` panics.
    Segment(Direction, Direction),
    /// The tail sprites.
    ///
    /// `Direction` is the direction in which the rest of the body
    /// is pointing, meaning **opposite** of the tail's point.
    Tail(Direction),
    Rabit,
    Grass,
}

impl From<&Sprite> for DrawParam {
    /// Creates a `DrawParam` with the correct `src` and `rotation` for
    /// the asked for `Sprite`.
    ///
    /// Will panic if the directions in the `Sprite::Segment` are not possible.
    fn from(sprite: &Sprite) -> DrawParam {
        let src: Rect = match sprite {
            Sprite::Head(direction) => match direction {
                Direction::Right => Rect::new(0.25, 0.0, 0.25, 0.25),
                Direction::Up => Rect::new(0.0, 0.0, 0.25, 0.25),
                Direction::Left => Rect::new(0.75, 0.0, 0.25, 0.25),
                Direction::Down => Rect::new(0.5, 0.0, 0.25, 0.25),
            },
            Sprite::Segment(src, tgt) => match (src, tgt) {
                (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => {
                    Rect::new(0.25, 0.75, 0.25, 0.25)
                }
                (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) => {
                    Rect::new(0.0, 0.75, 0.25, 0.25)
                }
                (Direction::Up, Direction::Right) | (Direction::Right, Direction::Up) => {
                    Rect::new(0.0, 0.5, 0.25, 0.25)
                }
                (Direction::Down, Direction::Right) | (Direction::Right, Direction::Down) => {
                    Rect::new(0.25, 0.5, 0.25, 0.25)
                }
                (Direction::Left, Direction::Down) | (Direction::Down, Direction::Left) => {
                    Rect::new(0.5, 0.5, 0.25, 0.25)
                }
                (Direction::Left, Direction::Up) | (Direction::Up, Direction::Left) => {
                    Rect::new(0.75, 0.5, 0.25, 0.25)
                }
                _ => panic!("Segment with same directions: {:?}", src),
            },
            Sprite::Tail(direction) => match direction {
                Direction::Up => Rect::new(0.0, 0.25, 0.25, 0.25),
                Direction::Right => Rect::new(0.25, 0.25, 0.25, 0.25),
                Direction::Down => Rect::new(0.5, 0.25, 0.25, 0.25),
                Direction::Left => Rect::new(0.75, 0.25, 0.25, 0.25),
            },
            Sprite::Rabit => Rect::new(0.5, 0.75, 0.25, 0.25),
            Sprite::Grass => Rect::new(0.75, 0.75, 0.25, 0.25),
        };

        DrawParam::default()
            .src(src)
            .scale([SPRITE_CELL_RATIO.0, SPRITE_CELL_RATIO.1])
    }
}

/// Contains all the information needed to
/// describe and display a sprite.
pub struct PositionedSprite {
    pub sprite: Sprite,
    pub position: GridPosition,
}

impl PositionedSprite {
    pub fn new(sprite: Sprite, position: GridPosition) -> Self {
        Self { sprite, position }
    }

    pub fn to_draw_param(&self, update_ratio: f32) -> DrawParam {
        std::debug_assert!(0.0 <= update_ratio && update_ratio <= 1.0);

        let direction: Option<Direction> = match self.sprite {
            Sprite::Head(direction) => Some(direction),
            Sprite::Segment(_, direction) => Some(direction),
            Sprite::Tail(direction) => Some(direction),
            _ => None,
        };

        let offset: na::Vector2<f32> = match direction {
            Some(direction) =>  {
                let vec: Vector2<f32> =  direction.into();
                na::Vector2::new(
                    vec.x * update_ratio * GRID_CELL_SIZE.0 as f32,
                    vec.y * update_ratio * GRID_CELL_SIZE.1 as f32,
                )
            }
            None => [0.0, 0.0].into(),
        };

        let param = DrawParam::from(self);
        let dest: na::Point2<f32> = param.dest.into();
        param.dest(dest + offset)
    }
}

impl From<&PositionedSprite> for DrawParam {
    fn from(ps: &PositionedSprite) -> Self {
        DrawParam::from(&ps.sprite).dest(ps.position)
    }
}
