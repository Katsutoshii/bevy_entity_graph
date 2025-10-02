use bevy::{
    app::{App, FixedUpdate, Plugin},
    ecs::{
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        schedule::{
            InternedSystemSet, IntoScheduleConfigs, ScheduleConfigs, SystemSet,
            common_conditions::on_event,
        },
        system::Query,
    },
    log::warn,
};

pub use crate::{
    connected_components::{ConnectedComponent, InConnectedComponent},
    connections::{ConnectionUpdateEvent, Connections},
};

mod connected_components;
mod connections;

pub struct EntityGraphPlugin;
impl Plugin for EntityGraphPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ConnectEvent>()
            .add_event::<DisconnectEvent>()
            .configure_sets(FixedUpdate, EntityGraphSystem::get_config())
            .add_systems(
                FixedUpdate,
                (
                    ConnectEvent::on_event.run_if(on_event::<ConnectEvent>),
                    DisconnectEvent::on_event.run_if(on_event::<DisconnectEvent>),
                )
                    .chain()
                    .in_set(EntityGraphSystem::UpdateGraph),
            )
            .add_plugins((
                connected_components::ConnectedComponentPlugin,
                connections::ConnectionsPlugin,
            ));
    }
}

/// Set enum for the systems relating to transform propagation
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum EntityGraphSystem {
    UpdateGraph,
    ComputeConnectedComponents,
}
impl EntityGraphSystem {
    fn get_config() -> ScheduleConfigs<InternedSystemSet> {
        (Self::UpdateGraph, Self::ComputeConnectedComponents).chain()
    }
}

#[derive(Event, Debug)]
pub struct ConnectEvent(pub (Entity, Entity));
impl ConnectEvent {
    fn on_event(
        mut events: EventReader<ConnectEvent>,
        mut updates: EventWriter<ConnectionUpdateEvent>,
        mut query: Query<&mut Connections>,
    ) {
        for &ConnectEvent((entity1, entity2)) in events.read() {
            if let Ok(mut connections) = query.get_mut(entity1) {
                connections.connect(entity2);
                updates.write(ConnectionUpdateEvent(entity1));
            } else {
                warn!("Entity {entity1} did not exist");
            }
            if let Ok(mut connections) = query.get_mut(entity2) {
                connections.connect(entity1);
                updates.write(ConnectionUpdateEvent(entity2));
            } else {
                warn!("Entity {entity2} did not exist");
            }
        }
    }
}

#[derive(Event, Debug)]
pub struct DisconnectEvent(pub (Entity, Entity));
impl DisconnectEvent {
    fn on_event(
        mut events: EventReader<DisconnectEvent>,
        mut updates: EventWriter<ConnectionUpdateEvent>,
        mut query: Query<&mut Connections>,
    ) {
        for &DisconnectEvent((entity1, entity2)) in events.read() {
            if let Ok(mut connections) = query.get_mut(entity1) {
                connections.disconnect(entity2);
                updates.write(ConnectionUpdateEvent(entity1));
            }
            if let Ok(mut connections) = query.get_mut(entity2) {
                connections.disconnect(entity1);
                updates.write(ConnectionUpdateEvent(entity2));
            }
        }
    }
}
