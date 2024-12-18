use ggez::Context;

use crate::level::Level;

pub trait Entity<AccessType: ?Sized = Self> {
    fn access_value(level: &mut Level) -> &mut AccessType
    where
        Self: Sized;

    /// Returns the implementor of Entity if it can be found in Level.
    fn access(level: &mut Level) -> Option<&mut AccessType>
    where
        Self: Sized;
}

impl<T: Entity> Entity<T> for Option<T> {
    fn access(level: &mut Level) -> Option<&mut T>
    where
        Self: Sized,
    {
        level.get::<T>()
    }
    
    fn access_value(level: &mut Level) -> &mut T
    where
        Self: Sized {
        level.get_opt::<T>()
    }
}

/// Can only run if the Entity is stored optionally for the level
pub trait ProcUpdate: Entity {
    fn update(&mut self, level: &mut Level, ctx: &mut Context);
}

/// Can run regardless of whether the Entity is optional for the level
pub trait GlobalUpdate<T>: Entity<T> {
    fn update(level: &mut Level, ctx: &mut Context)
    where
        Self: Sized;
}

impl<T: ProcUpdate + Entity<T>> GlobalUpdate<T> for Option<T> {
    fn update(level: &mut Level, ctx: &mut Context) {
        let get = level.get_opt::<T>();
        let entity = get.take();
        if let Some(mut entity) = entity {
            entity.update(level, ctx);
            level.get_opt().replace(entity);
        }
    }
}
