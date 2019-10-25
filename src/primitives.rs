use ggez::{
    event::{Axis, Button, KeyCode},
    mint::Point2,
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
#[derive(PartialEq, Eq, Clone, Copy)]
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
        Self { x: x, y: y }
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
