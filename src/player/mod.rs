use crate::collision::Hitbox;
use crate::item::ItemType;
use crate::level::{Level, ProtagData};
use crate::sword::Sword;
use crate::Direction;
use controller::ProtagController;
use ggez::graphics::{Canvas, Color};
use ggez::{Context, GameResult};
use glam::Vec2;
use inventory::Inventory;

mod controller;
mod inventory;

pub struct Protag {
    pub position: glam::Vec2,
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

        controller::ProtagController::update(&mut level.protag, ctx);
    }

    pub fn draw(level: &mut Level, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        level.protag.hurtbox.draw(
            &mut ctx.gfx,
            canvas,
            level.protag.position,
            Some(Color::RED),
        )?;

        Inventory::draw(level, canvas);

        Ok(())
    }
}
