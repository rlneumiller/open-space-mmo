fn setup_client() {
    let client = RenetClient::new(ConnectionConfig::default());
    let transport = NetcodeClientTransport::new(client_config, socket).unwrap();
    app.insert_resource(client)
       .insert_resource(transport);
}