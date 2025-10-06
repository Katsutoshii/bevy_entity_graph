use bevy::{
    app::{App, FixedUpdate, Plugin},
    ecs::{
        entity::Entity,
        message::{Message, MessageReader, MessageWriter},
        schedule::{
            InternedSystemSet, IntoScheduleConfigs, ScheduleConfigs, SystemSet,
            common_conditions::on_message,
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
        app.add_message::<ConnectMessage>()
            .add_message::<DisconnectMessage>()
            .configure_sets(FixedUpdate, EntityGraphSystem::get_config())
            .add_systems(
                FixedUpdate,
                (
                    ConnectMessage::on_message.run_if(on_message::<ConnectMessage>),
                    DisconnectMessage::on_message.run_if(on_message::<DisconnectMessage>),
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

#[derive(Message, Debug)]
pub struct ConnectMessage(pub (Entity, Entity));
impl ConnectMessage {
    fn on_message(
        mut events: MessageReader<ConnectMessage>,
        mut updates: MessageWriter<ConnectionUpdateEvent>,
        mut query: Query<&mut Connections>,
    ) {
        for &ConnectMessage((entity1, entity2)) in events.read() {
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

#[derive(Message, Debug)]
pub struct DisconnectMessage(pub (Entity, Entity));
impl DisconnectMessage {
    fn on_message(
        mut events: MessageReader<DisconnectMessage>,
        mut updates: MessageWriter<ConnectionUpdateEvent>,
        mut query: Query<&mut Connections>,
    ) {
        for &DisconnectMessage((entity1, entity2)) in events.read() {
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
