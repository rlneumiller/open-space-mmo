use bevy::prelude::*;
use bevy_renet::netcode::NetcodeClientTransport;
use bevy_renet::renet::{ConnectionConfig, DefaultChannel, RenetClient};
use renet_netcode::ClientAuthentication;
use std::net::UdpSocket;
use std::time::{Duration, SystemTime};

fn main() {
    let (client, transport) = create_renet_client();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_renet::RenetClientPlugin)
        .insert_resource(client)
        .insert_resource(transport)
        .add_systems(Startup, setup_client)
        .add_systems(Update, client_update_system)
        .run();
}

fn create_renet_client() -> (RenetClient, NetcodeClientTransport) {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: 0,
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    let client = RenetClient::new(ConnectionConfig::default());

    (client, transport)
}

fn setup_client(mut commands: Commands) {
    // Spawn 2D camera
    commands.spawn((
        Camera2d,
        Transform::default(),
    ));

    // Spawn text entity
    commands.spawn((
        Text::new("Client - Connecting..."),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn client_update_system(
    mut client: ResMut<RenetClient>,
    mut transport: ResMut<NetcodeClientTransport>,
    mut text_query: Query<&mut Text>,
) {
    let dt = Duration::from_secs_f32(1.0 / 60.0); // Fixed timestep for simplicity

    // Update client
    client.update(dt);
    transport.update(dt, &mut client).unwrap();

    // Update UI
    if let Ok(mut text) = text_query.single_mut() {
        let status = if client.is_connected() {
            "Connected"
        } else {
            "Connecting..."
        };
        *text = Text::new(format!("Client - {}", status));
    }

    // Handle messages
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        println!("Received message: {:?}", message);
    }
}
