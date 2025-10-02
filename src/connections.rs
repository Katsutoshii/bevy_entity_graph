use bevy::{
    app::{App, Plugin},
    ecs::{
        component::{Component, HookContext},
        entity::Entity,
        event::Event,
        world::DeferredWorld,
    },
    prelude::{Deref, DerefMut},
    reflect::Reflect,
};
use smallvec::SmallVec;

pub struct ConnectionsPlugin;
impl Plugin for ConnectionsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Connections>()
            .add_event::<ConnectionUpdateEvent>();
    }
}

#[derive(Component, Debug, Default, DerefMut, Deref, Reflect)]
#[component(on_remove = Connections::on_remove)]
pub struct Connections(pub SmallVec<[Entity; 10]>);
impl Connections {
    pub fn connect(&mut self, entity: Entity) {
        if self.0.contains(&entity) {
            return;
        }
        self.0.push(entity);
    }

    pub fn disconnect(&mut self, entity: Entity) {
        self.0.retain(|&mut e| e != entity);
    }

    pub fn on_remove(mut world: DeferredWorld, context: HookContext) {
        let connections = world.get::<Connections>(context.entity).unwrap().0.clone();
        for &entity in connections.iter() {
            world
                .get_mut::<Connections>(entity)
                .unwrap()
                .disconnect(context.entity);
        }
    }
}

/// Event alerting that an entity had its connection updated.
#[derive(Event, Debug)]
pub struct ConnectionUpdateEvent(pub Entity);
