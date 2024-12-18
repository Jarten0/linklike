use crate::ecs::Entity;
use crate::enemies::{basic_enemy::BasicEnemy, Enemy, EnemyContainer};
use crate::player::Protag;
use ggez::graphics::Canvas;
use ggez::{Context, GameResult};
use glam::Vec2;

pub struct Level {
    pub protag: Protag,
    pub enemies: EnemyContainer,
}

impl Level {
    pub fn new(ctx: &mut Context) -> Self {
        let data: LevelData = LevelData {
            protag: ProtagData {
                start_pos: Vec2::ONE * 500.0,
            },
            enemies: vec![<BasicEnemy as Enemy>::create],
        };

        let mut level = Self {
            protag: Protag::new(&data.protag, ctx),
            enemies: EnemyContainer::new(),
        };

        EnemyContainer::init(&mut level, &data, ctx);

        level
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        Protag::update(self, ctx);

        EnemyContainer::update(self, ctx);

        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        Protag::draw(self, canvas);

        Ok(())
    }

    pub fn get<T: Entity>(&mut self) -> Option<&mut T> {
        T::access(self)
    }

    pub fn get_opt<T: Entity>(&mut self) -> &mut Option<T> {
        T::access_value(self)
    }
}

pub struct LevelData {
    pub protag: ProtagData,
    pub enemies: Vec<fn(&mut Level, &mut Context) -> GameResult<Box<dyn Enemy>>>,
}

pub struct ProtagData {
    pub start_pos: glam::Vec2,
}
