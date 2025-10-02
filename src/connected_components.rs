use std::collections::VecDeque;

use bevy::{
    app::{App, FixedUpdate, Plugin},
    ecs::{
        component::Component,
        entity::{Entity, EntityHashSet},
        error::Result,
        event::EventReader,
        name::Name,
        reflect::ReflectComponent,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query},
    },
    log::warn,
    platform::collections::HashSet,
    prelude::Deref,
    reflect::Reflect,
};

use crate::{ConnectionUpdateEvent, Connections, EntityGraphSystem};

/// Plugin for tracking connected components of all entities in the graph.
pub struct ConnectedComponentPlugin;
impl Plugin for ConnectedComponentPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<HashSet<Entity>>()
            .register_type::<ConnectedComponent>()
            .register_type::<InConnectedComponent>()
            .add_systems(
                FixedUpdate,
                (InConnectedComponent::update, ConnectedComponent::update)
                    .chain()
                    .in_set(EntityGraphSystem::ComputeConnectedComponents),
            );
    }
}
/// Tracks which connected component this entity is in.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Connections)]
#[relationship(relationship_target = ConnectedComponent)]
pub struct InConnectedComponent(pub Entity);
impl InConnectedComponent {
    /// Get or spawn a connected component for an entity.
    pub fn get_or_spawn_connected_component(
        entity: Entity,
        in_connected_components: &Query<Option<&InConnectedComponent>>,
        used_components: &mut EntityHashSet,
        attached_to: &Connections,
        commands: &mut Commands,
    ) -> Entity {
        // First try getting a connected component from the immediate entity.
        if let Ok(Some(in_connected_component)) = in_connected_components.get(entity) {
            if used_components.insert(in_connected_component.0) {
                return in_connected_component.0;
            }
        }
        // If that doesn't exist, get from attachment
        for &attachment in attached_to.iter() {
            if let Ok(Some(in_connected_component)) = in_connected_components.get(attachment) {
                if used_components.insert(in_connected_component.0) {
                    return in_connected_component.0;
                }
            }
        }
        // Otherwise, spawn a new one.
        let entity = commands.spawn(ConnectedComponent::default()).id();
        used_components.insert(entity);
        entity
    }

    /// Maintain connected component invariants.
    /// After connections are updated, connected components are all invalid!
    /// Can we treat each update as if it were incremental?
    /// No, because the connections were all updated simultaneously. We would have needed the previous connections.
    /// Consider the case where we have a large component that has been split into two.
    /// The first component should reuse the ID, then saturate.
    /// The second component will try to reuse the ID, realize it's already in use, and then spawn a new connected component.
    pub fn update(
        mut updates: EventReader<ConnectionUpdateEvent>,
        in_connected_components: Query<Option<&InConnectedComponent>>,
        connections: Query<&Connections>,
        mut commands: Commands,
    ) -> Result {
        let mut visited = EntityHashSet::new();
        let mut used_components = EntityHashSet::new();
        let mut updated = EntityHashSet::new();

        for &ConnectionUpdateEvent(entity) in updates.read() {
            updated.insert(entity);
        }

        // Go through all entities that have connections updated.
        for entity in updated {
            let Ok(conns) = connections.get(entity) else {
                continue;
            };
            // Entities with no connections have no connected component.
            if conns.is_empty() {
                commands.entity(entity).remove::<InConnectedComponent>();
                continue;
            }

            // Skip already assigned entities.
            if visited.contains(&entity) {
                continue;
            }
            let connected_component = Self::get_or_spawn_connected_component(
                entity,
                &in_connected_components,
                &mut used_components,
                conns,
                &mut commands,
            );

            // BFS through all neighbor components.
            let mut queue = VecDeque::<Entity>::new();
            queue.push_front(entity);
            while let Some(entity) = queue.pop_back() {
                if !visited.insert(entity) {
                    continue;
                }
                let Ok(mut entity_commands) = commands.get_entity(entity) else {
                    warn!("Invalid entity in connections: {entity:?}");
                    continue;
                };
                entity_commands.insert(InConnectedComponent(connected_component));
                // Enqueue all attachments.
                if let Ok(conns) = connections.get(entity) {
                    for &next in conns.iter().filter(|&entity| !visited.contains(entity)) {
                        queue.push_front(next);
                    }
                }
            }
        }
        Ok(())
    }
}

/// Maintains a list of all entities in a connected component.
#[derive(Component, Reflect, Debug, Default, Deref)]
#[reflect(Component)]
#[require(Name::new("ConnectedComponent"))]
#[relationship_target(relationship = InConnectedComponent)]
pub struct ConnectedComponent(EntityHashSet);
impl ConnectedComponent {
    pub fn update(query: Query<(Entity, &ConnectedComponent)>, mut commands: Commands) {
        for (entity, connected_component) in query.iter() {
            if connected_component.0.is_empty() {
                commands.entity(entity).despawn();
            }
        }
    }
}
