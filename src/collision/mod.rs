use ggez::graphics::Rect;
use glam::Vec2;

use crate::Direction;

// Hitboxes can be rotated around an origin
#[derive(Debug, Clone)]
pub struct Hitbox {
    pub(crate) rect: Rect,
    pub(crate) direction: Direction,
}

impl Hitbox {
    pub fn new(rect: Rect) -> Self {
        Self {
            rect,
            direction: Direction::Right,
        }
    }

    /// Creates a new hitbox, centered at (0,0) + point, with the width and height both equalling the size divided by two.
    ///
    /// Exists as a helper function to make hitbox creation more intuitive
    pub fn point_size(point: Vec2, size: f32) -> Self {
        Self {
            rect: Rect {
                x: 0.0,
                y: 0.0,
                w: 0.0,
                h: 0.0,
            },
            direction: Direction::Right,
        }
    }

    /// Rotates the hitbox so that it points in the given direction.
    ///
    /// Does nothing if the hitbox is already pointing in that direction.
    pub fn as_direction(self, direction: Direction) {
        self.rect.rotate(direction.to_vec().);
    }

    pub fn colliding(&self, other: &Hitbox, offset: Vec2, other_offset: Vec2) -> bool {
        Rect::from([
            self.rect.x + offset.x,
            self.rect.y + offset.y,
            self.rect.w,
            self.rect.h,
        ])
        .overlaps(&Rect::from([
            other.rect.x + other_offset.x,
            other.rect.y + other_offset.y,
            other.rect.w,
            other.rect.h,
        ]))
    }

    // pub fn draw(&self, )
}
