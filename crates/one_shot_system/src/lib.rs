use std::collections::HashMap;

use bevy::{ecs::system::SystemId, prelude::*};

#[derive(Resource, Deref, DerefMut, Default)]
pub struct GameOneShotSystems(pub HashMap<String, SystemId>);

pub struct GameOneShotSystemBuilder<'a>(GameOneShotSystems, &'a mut World);

impl<'a> GameOneShotSystemBuilder<'a> {
    pub fn new(world: &'a mut World) -> Self {
        Self(GameOneShotSystems::default(), world)
    }
    pub fn add_system<M>(
        mut self,
        system: impl IntoSystem<(), (), M> + 'static,
        name: impl Into<String>,
    ) -> Self {
        let system_id = self.1.register_system(system);
        self.0.insert(name.into(), system_id);
        self
    }
}
