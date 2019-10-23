use ggez::event;
use ggez::graphics;
use ggez::graphics::Mesh;
use ggez::mint::{Point2, Vector2};
use ggez::{Context, GameResult};

#[derive(Debug, Clone)]
pub enum SnakeError {
    LogicError(String),
}
pub type SnakeResult<T = ()> = Result<T, SnakeError>;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn velocity(&self) -> Vector2<i32> {
        match self {
            Direction::Up => Vector2 { x: 0, y: -1 },
            Direction::Down => Vector2 { x: 0, y: 1 },
            Direction::Left => Vector2 { x: -1, y: 0 },
            Direction::Right => Vector2 { x: 1, y: 0 },
        }
    }

    pub fn inverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left
        }
    }

    pub fn from(key_code: event::KeyCode) -> Option<Self> {
        match key_code {
            event::KeyCode::Up => Some(Direction::Up),
            event::KeyCode::Down => Some(Direction::Down),
            event::KeyCode::Left => Some(Direction::Left),
            event::KeyCode::Right => Some(Direction::Right),
            _ => None,
        }
    }
}

pub struct Snake {
    mesh: Mesh,
    direction: Direction,
    direction_cache: Direction,
    points: Vec<Point2<i32>>,
}

impl Snake {
    pub fn new(mesh: Mesh, position: Point2<i32>, direction: Direction) -> Self {
        Self {
            mesh: mesh,
            direction: direction,
            direction_cache: direction,
            points: vec![position],
        }
    }

    pub fn set_direction(&mut self, direction: Direction) -> SnakeResult<()> {
        if direction != self.direction_cache.inverse() {
            self.direction = direction;
            Ok(())
        } else {
            Err(SnakeError::LogicError(
                "Can only update direction if it is orthogonal to previous direction".to_owned(),
            ))
        }
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn position(&self) -> Point2<i32> {
        *self.points.last().expect("Snake must not be empty")
    }

    pub fn points(&self) -> &[Point2<i32>] {
        &self.points
    }

    pub fn update(&mut self, grow: bool) {
        let velocity = self.direction.velocity();

        let head_point = *self.points.last().expect("Snake must not be empty");
        self.points.push(Point2 {
            x: head_point.x + velocity.x,
            y: head_point.y + velocity.y,
        });
        if !grow {
            self.points.remove(0);
        }
        self.direction_cache = self.direction;
    }

    pub fn draw(&mut self, ctx: &mut Context, cell_size: (i32, i32)) -> GameResult {
        for point in &self.points {
            graphics::draw(
                ctx,
                &mut self.mesh,
                (Point2 {
                    x: (point.x * cell_size.0) as f32,
                    y: (point.y * cell_size.1) as f32,
                },),
            )?;
        }
        Ok(())
    }
}

pub struct Food {
    mesh: Mesh,
    position: Point2<i32>,
}

impl Food {
    pub fn new(mesh: Mesh, position: Point2<i32>) -> Self {
        Self {
            mesh: mesh,
            position: position,
        }
    }

    pub fn set_position(&mut self, position: Point2<i32>) {
        self.position = position;
    }

    pub fn position(&self) -> Point2<i32> {
        self.position
    }

    pub fn draw(&mut self, ctx: &mut Context, cell_size: (i32, i32)) -> GameResult {
        graphics::draw(
            ctx,
            &mut self.mesh,
            (Point2 {
                x: (self.position.x * cell_size.0) as f32,
                y: (self.position.y * cell_size.1) as f32,
            },),
        )?;
        Ok(())
    }
}
