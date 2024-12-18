use super::Enemy;
use crate::level::Level;
use ggez::{Context, GameResult};
pub struct BasicEnemy {}

impl Enemy for BasicEnemy {
    fn create(level: &mut Level, ctx: &mut Context) -> GameResult<Box<dyn Enemy>>
    where
        Self: Sized,
    {
        Ok(Box::new(BasicEnemy {}))
    }

    fn update(&mut self, level: &mut Level, ctx: &mut Context) -> GameResult {
        todo!()
    }

    fn draw(&mut self, level: &Level, ctx: &mut Context) -> GameResult {
        todo!()
    }
}
