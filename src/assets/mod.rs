use glam::Vec2;
use player::{InventoryData, ProtagData, SwordData};

pub mod player;

/// Anything can borrow this thing for 'level.
#[derive(Debug)]
pub struct StaticAssets {
    pub protag: ProtagData,
}

impl StaticAssets {
    pub(crate) fn new() -> Self {
        Self {
            protag: ProtagData {
                start_pos: Vec2::ONE * 500.0,
                inventory: InventoryData {
                    sword: SwordData {},
                },
            },
        }
    }
}

// impl Default for &'static StaticAssets {
//     fn default() -> Self {
//         panic!("This is a cheat for Reflect; do not initialize &'static StaticAssets using Default or Reflect")
//     }
// }
