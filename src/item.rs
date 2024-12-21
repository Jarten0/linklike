use bevy_reflect::Reflect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Reflect)]
pub enum ItemType {
    #[default]
    None,
    Sword,
    Boomerang,
    Bow,
    Bomb,
}
