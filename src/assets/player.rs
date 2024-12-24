use std::convert::AsRef;

use ggez::graphics::Rect;
use glam::Vec2;

use crate::collision::{Hitbox, HitboxFrameRef, HitboxFrameStringRef, StaticHitboxFrameString};
use crate::Direction;

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
    swing: [StaticHitboxFrameString; 4],
}

pub fn new() -> ProtagData {
    let hitboxes = [Hitbox::new(Rect::one())];
    let hitbox_frames = &[&HitboxFrameRef::new(&hitboxes)];
    let hitbox_frame_string = HitboxFrameStringRef::new(hitbox_frames);

    let collect: Vec<Vec<Hitbox>> = HitboxFrameStringRef::as_direction::<
        Vec<Vec<Hitbox>>,
        Vec<Hitbox>,
    >(hitbox_frame_string, Direction::Right);

    let directions = [
        HitboxFrameStringRef::new(&collect),
        // hitbox_frame_string.as_direction(Direction::Right),
        // hitbox_frame_string.as_direction(Direction::Right),
        // hitbox_frame_string.as_direction(Direction::Right),
    ];
    // let strings = [
    //     // HitboxFrameString::new(&di)
    // ]
    ProtagData {
        inventory: InventoryData {
            sword: SwordData {
                swing: [
                    todo!(),
                    todo!(),
                    todo!(),
                    todo!(),
                    // HitboxFrameString::new(&strings)
                ],
            },
        },
        start_pos: Vec2::ONE * 500.0,
    }
}
