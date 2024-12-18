use crate::level::{Level, LevelData};
use ggez::{Context, GameResult};
use std::collections::LinkedList;

pub mod basic_enemy;

pub trait Enemy {
    fn create(level: &mut Level, ctx: &mut Context) -> GameResult<Box<dyn Enemy>>
    where
        Self: Sized;

    fn update(&mut self, level: &mut Level, ctx: &mut Context) -> GameResult;

    fn draw(&mut self, level: &Level, ctx: &mut Context) -> GameResult;
}

pub struct EnemyContainer {
    pub enemies: LinkedList<Option<Box<dyn Enemy>>>,
}

impl EnemyContainer {
    pub fn new() -> Self {
        Self {
            enemies: LinkedList::new(),
        }
    }

    pub fn init(level: &mut Level, level_data: &LevelData, ctx: &mut Context) {
        for enemy_data in level_data.enemies.iter() {
            let enemy_result = enemy_data(level, ctx)
                .err()
                .map(|err| panic!("Enemy init failed [{}]", err));
            level.enemies.enemies.push_back(enemy_result);
        }
    }

    pub(crate) fn update(level: &mut Level, ctx: &mut Context) -> GameResult {
        todo!()
    }
}
