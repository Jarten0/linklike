use bevy_reflect::Reflect;

pub mod sword;

#[derive(Debug, Clone, Copy, Default, PartialEq, Reflect)]
pub enum ItemType {
    #[default]
    None,
    Sword,
    Boomerang,
    Bow,
    Bomb,
}

/// Hooks that Items can use to communicate details to the player
pub trait ProtagItem {
    /// If false, then the item can be switched out and does not require updates to be locked to this item.
    fn active(&mut self) -> bool;

    /// If false, then the player will not be able to move while the item is active.
    /// This has no effect if the item is inactive.
    ///
    /// See also [`ProtagItem::can_turn`]
    fn can_move(&mut self) -> bool {
        true
    }

    /// If false, then the player will not be able to turn while the item is active.
    /// This has no effect if the item is inactive.
    ///
    /// See also [`ProtagItem::can_move`]
    fn can_turn(&mut self) -> bool {
        true
    }
}
