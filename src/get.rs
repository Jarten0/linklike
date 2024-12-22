use bevy_reflect::{GetField, PartialReflect, Reflect, ReflectMut, ReflectRef};

/// Returns an item with the given type from the struct,
pub trait Get<Item> {
    fn get<'a>(&'a self, _id: &'static str) -> Option<&'a Item>;
    fn get_mut<'a>(&'a mut self, _id: &'static str) -> Option<&'a mut Item>;
}

/// Get `Self` from the `Container`.
pub trait Access<Container> {
    fn access<'get>(access: &'get Container, id: &'static str) -> Option<&'get Self>;
    fn access_mut<'get>(access: &'get mut Container, id: &'static str) -> Option<&'get mut Self>;
}

impl<Container: Reflect, Item: Reflect> Get<Item> for Container {
    fn get<'a>(&'a self, id: &'static str) -> Option<&'a Item> {
        match self.reflect_ref() {
            ReflectRef::Struct(s) => s.get_field(id),
            _ => None,
        }
    }

    fn get_mut<'a>(&'a mut self, _id: &'static str) -> Option<&'a mut Item> {
        Item::access_mut(self, _id)
    }
}

impl<C: Reflect, I: Reflect> Access<C> for I {
    fn access<'get>(access: &'get C, id: &'static str) -> Option<&'get I> {
        match access.reflect_ref() {
            ReflectRef::Struct(s) => s
                .field(id)
                .map(<dyn PartialReflect>::try_downcast_ref)
                .unwrap_or_default(),
            _ => None,
        }
    }

    fn access_mut<'get>(access: &'get mut C, id: &'static str) -> Option<&'get mut Self> {
        match access.reflect_mut() {
            ReflectMut::Struct(s) => s.get_field_mut(id),
            // .map(<dyn PartialReflect>::try_downcast_mut)
            // .unwrap_or_default(),
            _ => None,
        }
    }
}
