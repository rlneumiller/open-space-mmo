fn setup_server() {
    let server = RenetServer::new(ConnectionConfig::default());
    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    app.insert_resource(server)
       .insert_resource(transport);
}