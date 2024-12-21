use bevy_reflect::Reflect;
use ggez::graphics::Canvas;
use ggez::Context;

use crate::item::ItemType;
use crate::level::Level;
use crate::sword::Sword;

#[derive(Debug, Reflect)]
pub struct Inventory {
    #[reflect(ignore)]
    pub sword: Sword,
    // #[reflect(ignore)]
    pub current_item: ItemType,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            current_item: ItemType::Sword,
            sword: Sword::new(),
        }
    }

    pub fn draw(level: &mut Level, ctx: &mut Context, canvas: &mut Canvas) {
        match level.protag.inventory.current_item {
            ItemType::None => (),
            ItemType::Sword => Sword::draw(level, ctx, canvas),
            ItemType::Boomerang => todo!(),
            ItemType::Bow => todo!(),
            ItemType::Bomb => todo!(),
        };
    }
}
