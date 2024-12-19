use ggez::graphics::Canvas;

use crate::item::ItemType;
use crate::level::Level;
use crate::sword::Sword;

pub struct Inventory {
    pub sword: Sword,
    pub current_item: ItemType,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            current_item: ItemType::Sword,
            sword: Sword::new(),
        }
    }

    pub fn draw(level: &mut Level, canvas: &mut Canvas) {
        match level.protag.inventory.current_item {
            ItemType::None => (),
            ItemType::Sword => Sword::draw(level, canvas),
            ItemType::Boomerang => todo!(),
            ItemType::Bow => todo!(),
            ItemType::Bomb => todo!(),
        };
    }
}
