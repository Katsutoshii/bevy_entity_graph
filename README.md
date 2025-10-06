# `bevy_entity_graph`

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/Katsutoshii/bevy_entity_graph#license)
[![Crates.io](https://img.shields.io/crates/v/bevy_entity_graph.svg)](https://crates.io/crates/bevy_entity_graph)
[![Docs](https://docs.rs/bevy_entity_graph/badge.svg)](https://docs.rs/bevy_entity_graph/latest/bevy_entity_graph/)

Crate to facilitate working with graphs of entities in Bevy.

Features:
- Event-driven graph modifications.
- Auotmatically updated connected component tracking.

## Usage

```rs
use bevy_entity_graph::{ConnectMessage, Connections, EntityGraphPlugin, InConnectedComponent};

fn my_system(mut commands: Commands, mut connect_events: EventWriter<ConnectEvent>) {
  // Spawn some entities with Connections.
  let e1 = commands.spawn((Connections::default(), ...)).id();
  let e2 = commands.spawn((Connections::default(), ...)).id();
  // Connect them.
  connect_events.write(ConnectMessage((e1, e2)));
}
```

See `examples` for a working demo.

## Bevy support table

| bevy | bevy_entity_graph |
| ---- | ----------------- |
| 0.17 | 0.2.0             |
| 0.16 | 0.1.0             |
