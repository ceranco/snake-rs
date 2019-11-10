use crate::primitives::*;
use ggez::{
    graphics::{self, Mesh, Image},
    mint::Point2,
    Context, GameResult,
};
use std::collections::VecDeque;

/// Contains all the information needed to describe
/// the state of the snake itself.
pub struct Snake {
    head: GridPosition,
    body: VecDeque<GridPosition>,
    direction: Direction,
    previous_direction: Direction,
    next_direction: Option<Direction>,
}

impl Snake {
    /// Creates a new `Snake` with one head segment at the
    /// given position and one `Tail` segment behind it (direction is right).
    pub fn new(position: GridPosition) -> Self {
        let mut body = VecDeque::new();
        body.push_back(GridPosition::new_from_move(position, Direction::Left));
        Self {
            head: position,
            body: body,
            direction: Direction::Right,
            previous_direction: Direction::Right,
            next_direction: None,
        }
    }

    /// Sets the direction of the snake.
    ///
    /// If the direction is invalid, returns a `SnakeError::LogicError`.
    pub fn set_direction(&mut self, direction: Direction) -> SnakeResult<()> {
        let is_valid = self.direction.inverse() != direction;
        if is_valid {
            if self.previous_direction == self.direction {
                self.direction = direction;
            } else {
                self.next_direction = Some(direction);
            }
            Ok(())
        } else {
            Err(SnakeError::LogicError(
                "Can only update direction if it is orthogonal to previous direction".to_owned(),
            ))
        }
    }

    /// Returns a `Vec` of ***all*** of the segments of the
    /// snake (including the head).
    ///
    /// This is useful to generate a new position for the
    /// food and check that the snake isn't already there.
    pub fn segments(&self) -> Vec<GridPosition> {
        let mut vec: Vec<GridPosition> = self.body.iter().copied().collect();
        vec.push(self.head);
        vec
    }

    /// Helper function that checks if the `Snake`
    /// is eating the `Food` in its current state.
    fn eats_food(&self, food: &Food) -> bool {
        self.head == food.position()
    }

    /// Helper function that checks if the `Snake`
    /// is eating itself in its current state.
    fn eats_self(&self) -> bool {
        self.body.contains(&self.head)
    }

    /// Updates the state of the `Snake`.
    pub fn update(&mut self, food: &Food) -> Option<Ate> {
        // move in the set direction
        let new_head = GridPosition::new_from_move(self.head, self.direction);

        // update the "direction cache"
        self.previous_direction = self.direction;
        if let Some(next_direction) = self.next_direction {
            self.direction = next_direction;
            self.next_direction = None;
        }

        // push the current head to the body and update its position
        self.body.push_front(self.head);
        self.head = new_head;

        // check if the snake is eating something
        if self.eats_food(food) {
            Some(Ate::Food)
        } else {
            // if the snake didn't eat food, pop the last body segment
            // to create the illusion of movement
            self.body.pop_back();
            if self.eats_self() {
                Some(Ate::Itself)
            } else {
                None
            }
        }
    }

    /// Draws the `Snake` to the screen in its current state.
    pub fn draw(&mut self, ctx: &mut Context, sprites: &mut Image) -> GameResult {
        let mut point: Point2<f32> = self.head.into();
        graphics::draw(ctx, sprites, Sprite::Head(Direction::Right).get_param().dest(point))?;
        for segment in &self.body {
            point = (*segment).into();
            graphics::draw(ctx, sprites, Sprite::Tail(Direction::Right).get_param().dest(point))?;
        }
        Ok(())
    }
}

/// Represents a piece of food that the `Snake` can eat.
pub struct Food {
    position: GridPosition,
}

impl Food {
    pub fn new(position: GridPosition) -> Self {
        Self {
            position: position,
        }
    }

    pub fn position(&self) -> GridPosition {
        self.position
    }

    pub fn set_position(&mut self, position: GridPosition) {
        self.position = position;
    }

    pub fn draw(&mut self, ctx: &mut Context, sprites: &mut Image) -> GameResult {
        let point: Point2<f32> = self.position.into();
        graphics::draw(ctx, sprites, Sprite::Rabit.get_param().dest(point))?;
        Ok(())
    }
}
