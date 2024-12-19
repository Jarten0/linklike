use crate::level::{Level, LevelData};
use ggez::graphics::Canvas;
use ggez::{Context, GameResult};
use std::collections::LinkedList;

pub mod basic_enemy;

pub trait Enemy {
    fn create(level: &mut Level, ctx: &mut Context) -> GameResult<Box<dyn Enemy>>
    where
        Self: Sized;

    fn update(&mut self, level: &mut Level, ctx: &mut Context) -> GameResult;

    fn draw(&mut self, level: &Level, ctx: &mut Context, canvas: &mut Canvas) -> GameResult;
}

pub struct EnemyContainer {
    pub container: Vec<Option<Box<dyn Enemy>>>,
}

impl EnemyContainer {
    pub fn new() -> Self {
        Self {
            container: Vec::new(),
        }
    }

    pub fn init(level: &mut Level, level_data: &LevelData, ctx: &mut Context) {
        for enemy_data in level_data.enemies.iter() {
            let enemy_result = enemy_data(level, ctx)
                .err()
                .map(|err| panic!("Enemy init failed [{}]", err));
            level.enemies.container.push(enemy_result);
        }
    }

    pub(crate) fn update(level: &mut Level, ctx: &mut Context) -> GameResult {
        for i in 0..level.enemies.container.len() {
            let Some(mut enemy) = level.enemies.container[i].take() else {
                continue;
            };
            enemy.update(level, ctx)?;
            level.enemies.container[i].insert(enemy);
        }

        Ok(())
    }

    pub(crate) fn draw(level: &mut Level, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        for i in 0..level.enemies.container.len() {
            let Some(mut enemy) = level.enemies.container[i].take() else {
                continue;
            };
            enemy.draw(level, ctx, canvas)?;
            level.enemies.container[i].insert(enemy);
        }

        Ok(())
    }
}
