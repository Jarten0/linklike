use crate::collision::{Hitbox, HitboxFrameRef, HitboxFrameString, HitboxFrameStringRef};
use crate::Direction;
use glam::Vec2;
use std::borrow::Borrow;
use std::convert::AsRef;

#[derive(Debug, Clone)]
pub struct ProtagData {
    pub inventory: InventoryData,
    pub start_pos: glam::Vec2,
}

#[derive(Debug, Clone)]
pub struct InventoryData {
    pub sword: SwordData,
}

#[derive(Debug, Clone)]
pub struct SwordData {
    swing: [HitboxFrameString; 4],
}

impl ProtagData {
    pub fn new() -> ProtagData {
        static SWING_HITBOXES: HitboxFrameStringRef =
            HitboxFrameStringRef::new(&[HitboxFrameRef::new(&[
                Hitbox::point_size(Vec2::ZERO, 40.0),
                Hitbox::point_size(Vec2::ONE, 40.0),
                Hitbox::point_size(Vec2::new(100.0, 200.0), 40.0),
            ])]);

        let swing = [
            SWING_HITBOXES.to_direction(Direction::Right),
            SWING_HITBOXES.to_direction(Direction::Up),
            SWING_HITBOXES.to_direction(Direction::Left),
            SWING_HITBOXES.to_direction(Direction::Down),
        ];

        ProtagData {
            inventory: InventoryData {
                sword: SwordData { swing },
            },
            start_pos: Vec2::ONE * 500.0,
        }
    }
}
