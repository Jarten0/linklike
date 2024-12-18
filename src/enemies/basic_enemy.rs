use super::Enemy;
use crate::level::Level;
use ggez::graphics::{Canvas, Drawable, Quad, Rect};
use ggez::{Context, GameResult};

pub struct BasicEnemy {
    pub sprite: Quad,
    // pub
}

pub struct OverheadAttack {
    pub hitboxes: [Vec<Rect>],
}

// pub struct

impl Enemy for BasicEnemy {
    fn create(level: &mut Level, ctx: &mut Context) -> GameResult<Box<dyn Enemy>>
    where
        Self: Sized,
    {
        Ok(Box::new(BasicEnemy { sprite: Quad }))
    }

    fn update(&mut self, level: &mut Level, ctx: &mut Context) -> GameResult {
        todo!()
    }

    fn draw(&mut self, level: &Level, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        todo!()
    }
}
