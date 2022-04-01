use std::net::{Ipv4Addr, SocketAddrV4};
use bevy::prelude::*;
use bevy_osc::{OscDispatcher, OscUdpClient, OscUdpServer};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        .insert_resource(OscUdpServer::new("0.0.0.0:31337"))
        .insert_resource(OscDispatcher::default())
        .insert_resource(OscUdpClient::new(SocketAddrV4::new(Ipv4Addr::from([1,2,3,4]), 31337).into()))

        .run();
}
