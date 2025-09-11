use bevy::prelude::*;
use bevy_renet::netcode::NetcodeServerTransport;
use bevy_renet::renet::{ConnectionConfig, DefaultChannel, RenetServer, ServerEvent};
use renet_netcode::ServerAuthentication;
use std::net::UdpSocket;
use std::time::{Duration, SystemTime};

fn main() {
    let (server, transport) = create_renet_server();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_renet::RenetServerPlugin)
        .insert_resource(server)
        .insert_resource(transport)
        .add_systems(Startup, setup_server)
        .add_systems(Update, server_update_system)
        .run();
}

fn create_renet_server() -> (RenetServer, NetcodeServerTransport) {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

    let server_config = renet_netcode::ServerConfig {
        max_clients: 64,
        protocol_id: 0,
        public_addresses: vec![server_addr],
        authentication: ServerAuthentication::Unsecure,
        current_time,
    };

    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    let server = RenetServer::new(ConnectionConfig::default());

    (server, transport)
}

fn setup_server(mut commands: Commands) {
    // Spawn 2D camera
    commands.spawn((
        Camera2d,
        Transform::default(),
    ));

    // Spawn text entity
    commands.spawn((
        Text::new("Server - Running"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn server_update_system(
    mut server: ResMut<RenetServer>,
    mut transport: ResMut<NetcodeServerTransport>,
    mut text_query: Query<&mut Text>,
) {
    let dt = Duration::from_secs_f32(1.0 / 60.0); // Fixed timestep for simplicity

    // Update server
    server.update(dt);
    transport.update(dt, &mut server).unwrap();

    // Handle server events
    while let Some(event) = server.get_event() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client {} connected", client_id);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client {} disconnected: {:?}", client_id, reason);
            }
        }
    }

    // Update UI with client count
    if let Ok(mut text) = text_query.single_mut() {
        let client_count = server.clients_id().len();
        *text = Text::new(format!("Server - {} clients connected", client_count));
    }

    // Broadcast a simple message to all clients
    let message = format!("Server running with {} clients", server.clients_id().len());
    server.broadcast_message(DefaultChannel::ReliableOrdered, message.into_bytes());
}
