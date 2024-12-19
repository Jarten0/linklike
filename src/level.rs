use crate::collision::TEST_HITBOX_STRING;
use crate::enemies::{basic_enemy::BasicEnemy, Enemy, EnemyContainer};
use crate::player::Protag;
use ggez::graphics::{Canvas, Color};
use ggez::{Context, GameResult};
use glam::Vec2;

pub struct Level {
    pub protag: Protag,
    pub enemies: EnemyContainer,
    i: usize,
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
            i: 0,
        };

        EnemyContainer::init(&mut level, &data, ctx);

        level
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        Protag::update(self, ctx);

        EnemyContainer::update(self, ctx)?;

        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        Protag::draw(self, ctx, canvas)?;

        EnemyContainer::draw(self, ctx, canvas)?;

        crate::collision::TEST_HITBOX_STRING.draw(
            &mut ctx.gfx,
            canvas,
            self.i,
            Vec2::ONE * 59.0,
            Color::MAGENTA,
        )?;

        self.i += 1;
        if self.i >= TEST_HITBOX_STRING.len() {
            self.i = 0;
        }

        Ok(())
    }
}

pub struct LevelData {
    pub protag: ProtagData,
    pub enemies: Vec<fn(&mut Level, &mut Context) -> GameResult<Box<dyn Enemy>>>,
}

pub struct ProtagData {
    pub start_pos: glam::Vec2,
}
