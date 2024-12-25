#[derive(Debug, bevy_reflect::Reflect, serde::Serialize)]
pub struct Health(pub f32);

impl crate::npc::Stat for Health {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        "Health"
    }

    fn stat_name(&self) -> &'static str {
        "Health"
    }

    fn get(&self) -> &dyn bevy_reflect::Reflect {
        &self.0
    }

    fn get_mut(&mut self) -> &mut dyn bevy_reflect::Reflect {
        &mut self.0
    }
}

#[derive(Debug, bevy_reflect::Reflect, serde::Serialize)]
pub struct Damage(pub f32);

impl crate::npc::Stat for Damage {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        "Damage"
    }

    fn stat_name(&self) -> &'static str {
        "Damage"
    }

    fn get(&self) -> &dyn bevy_reflect::Reflect {
        &self.0
    }

    fn get_mut(&mut self) -> &mut dyn bevy_reflect::Reflect {
        &mut self.0
    }
}

#[derive(Debug, bevy_reflect::Reflect, serde::Serialize)]
pub struct Defense(pub f32);

impl crate::npc::Stat for Defense {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        "Defense"
    }

    fn stat_name(&self) -> &'static str {
        "Defense"
    }

    fn get(&self) -> &dyn bevy_reflect::Reflect {
        &self.0
    }

    fn get_mut(&mut self) -> &mut dyn bevy_reflect::Reflect {
        &mut self.0
    }
}
