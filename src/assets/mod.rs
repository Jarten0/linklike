use player::ProtagData;

use crate::npc::advanced_enemy::AdvancedEnemyData;

pub mod player;

/// Storage container for runtime generated data that lasts for the entirety of the program.
///
/// Anything can reference this for the length of `'static`, but only after it's been initialized and leaked.
/// Trying to access it before then will error.
///
/// If you're going to store a reference to this in a struct that's going to implement [`Reflect`](bevy_reflect::Reflect),
/// then you'll need to add the [`#[reflect(Default)]`](bevy_reflect::reflect::Reflect) struct attribute
/// (and add the [`bevy_reflect::std_traits::ReflectDefault`] import),
/// then provide a manual implementation of Default for your struct.
///
/// You can then use [`Game::static_assets`](crate::Game::static_assets) in your manual Default implementation
/// to get a reference to any data you'll need, and then add an initialization function if you want to further
/// initialize your data with any level-specific data.
#[derive(Debug)]
pub struct StaticAssets {
    pub protag: ProtagData,
    pub advanced_enemy: AdvancedEnemyData,
}

impl StaticAssets {
    pub(crate) fn new() -> Self {
        Self {
            protag: ProtagData::new(),
            advanced_enemy: AdvancedEnemyData::new(),
        }
    }
}
