//! Example to demonstrate creating a graph of entities and visualizing out their connections.
use bevy::{
    DefaultPlugins,
    app::{App, Startup, Update},
    asset::Assets,
    camera::{Camera2d, ClearColor},
    color::{
        Color,
        palettes::css::{BLUE, RED, WHITE},
    },
    ecs::{
        component::Component,
        message::MessageWriter,
        system::{Commands, Query, ResMut},
    },
    gizmos::gizmos::Gizmos,
    math::{Isometry2d, Vec3Swizzles, primitives::Circle},
    mesh::{Mesh, Mesh2d},
    sprite_render::{ColorMaterial, MeshMaterial2d},
    transform::components::{GlobalTransform, Transform},
};
use bevy_entity_graph::{ConnectMessage, Connections, EntityGraphPlugin, InConnectedComponent};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EntityGraphPlugin))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .add_systems(Update, draw_connections)
        .run();
}

#[derive(Component, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[require(Connections)]
struct Node(usize);

/// Spawn nodes.
fn setup(
    mut commands: Commands,
    mut connect_events: MessageWriter<ConnectMessage>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    let circle = meshes.add(Circle { radius: 10.0 });
    let e1 = commands
        .spawn((
            Node(1),
            Transform::from_xyz(0.0, 0.0, 0.0),
            Mesh2d(circle.clone()),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(WHITE))),
        ))
        .id();
    let e2 = commands
        .spawn((
            Node(2),
            Transform::from_xyz(60.0, 30.0, 0.0),
            Mesh2d(circle.clone()),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(WHITE))),
        ))
        .id();
    let e3 = commands
        .spawn((
            Node(3),
            Transform::from_xyz(-30.0, 90.0, 0.0),
            Mesh2d(circle.clone()),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(WHITE))),
        ))
        .id();
    let e4 = commands
        .spawn((
            Node(4),
            Transform::from_xyz(50.0, -40.0, 0.0),
            Mesh2d(circle.clone()),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(WHITE))),
        ))
        .id();
    let _ = commands
        .spawn((
            Node(5),
            Transform::from_xyz(30.0, 120.0, 0.0),
            Mesh2d(circle.clone()),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(WHITE))),
        ))
        .id();
    connect_events.write(ConnectMessage((e1, e2)));
    connect_events.write(ConnectMessage((e2, e3)));
    connect_events.write(ConnectMessage((e3, e4)));
}

/// Draw the connections between nodes.
fn draw_connections(
    mut gizmos: Gizmos,
    query: Query<(
        &GlobalTransform,
        &Connections,
        Option<&InConnectedComponent>,
    )>,
    transforms: Query<&GlobalTransform>,
) {
    for (transform, connections, in_connected_component) in query.iter() {
        for &other_entity in connections.iter() {
            let other_transform = transforms.get(other_entity).unwrap();
            gizmos.line_2d(
                transform.translation().xy(),
                other_transform.translation().xy(),
                WHITE,
            );
            if in_connected_component.is_some() {
                gizmos.circle_2d(
                    Isometry2d::from_translation(transform.translation().xy()),
                    10.,
                    RED,
                );
            } else {
                gizmos.circle_2d(
                    Isometry2d::from_translation(transform.translation().xy()),
                    10.,
                    BLUE,
                );
            }
        }
    }
}
