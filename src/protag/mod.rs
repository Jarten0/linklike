use crate::assets::player::ProtagData;
use crate::collision::Hitbox;
use crate::level::Level;
use crate::Direction;
use bevy_reflect::Reflect;
use controller::ProtagController;
use ggez::graphics::{Canvas, Color};
use ggez::{Context, GameResult};
use glam::Vec2;
use inventory::Inventory;
use items::sword::Sword;
use items::ItemType;

pub mod controller;
pub mod inventory;
pub mod items;

#[derive(Debug, Reflect)]
pub struct Protag {
    #[reflect(ignore)]
    pub position: glam::Vec2,
    #[reflect(ignore)]
    pub scale: glam::Vec2,
    pub direction: Direction,
    pub hurtbox: Hitbox,
    pub controller: ProtagController,
    pub inventory: Inventory,
}

impl Protag {
    pub fn new(init: &ProtagData, ctx: &mut Context) -> Self {
        Self {
            position: init.start_pos,
            direction: Direction::Down,
            inventory: inventory::Inventory::new(),
            controller: ProtagController::new(),
            scale: [80.0, 80.0].into(),
            hurtbox: Hitbox::point_size(Vec2::ZERO, 80.0),
        }
    }

    pub fn update(level: &mut Level, ctx: &mut Context) {
        match level.protag.inventory.current_item {
            ItemType::None => (),
            ItemType::Sword => Sword::update(level, ctx),
            ItemType::Boomerang => todo!(),
            ItemType::Bow => todo!(),
            ItemType::Bomb => todo!(),
        }

        controller::ProtagController::update(level, ctx);
    }

    pub fn draw(level: &mut Level, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        level.protag.hurtbox.draw(
            &mut ctx.gfx,
            canvas,
            level.protag.position,
            level.protag.controller.hurt.then_some(Color::RED),
        )?;

        Inventory::draw(level, ctx, canvas);

        Ok(())
    }
}
