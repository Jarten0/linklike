use std::any::{Any, TypeId};

use crate::assets::StaticAssets;
use crate::npc::{basic_enemy::BasicEnemy, Enemy, EnemyContainer};
use crate::protag::Protag;
use bevy_reflect::{GetField, PartialReflect, Reflect, ReflectMut, ReflectRef};
use ggez::graphics::{Canvas, Color};
use ggez::{Context, GameResult};
use glam::Vec2;

#[derive(Debug, Reflect)]
pub struct Level {
    pub protag: Protag,
    pub enemies: EnemyContainer,
    #[reflect(ignore)]
    #[reflect(default = "crate::Game::static_assets")]
    pub static_assets: &'static StaticAssets,
}

impl Level {
    pub fn initialize_assets(assets: &mut StaticAssets) {}

    pub fn new(ctx: &mut Context, assets: &'static StaticAssets) -> Self {
        // let data: LevelData = LevelData {
        //     protag: ProtagData {
        //         start_pos: Vec2::ONE * 500.0,
        //     },
        //     enemies: vec![(<BasicEnemy as Enemy>::create, "basic_enemy")],
        // };

        let mut level = Self {
            protag: Protag::new(&assets.protag, ctx),
            enemies: EnemyContainer::new(),
            static_assets: assets,
        };

        EnemyContainer::init(&mut level, ctx);

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

        Ok(())
    }
}
