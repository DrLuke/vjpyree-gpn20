use std::net::{Ipv4Addr, SocketAddrV4};
use bevy::prelude::*;
use bevy_osc::{OscDispatcher, OscMethod, OscMultiMethod, OscUdpClient, OscUdpServer};
use crate::pyree_modules::{pyree_startup_system_set, pyree_system_set};

mod pyree_modules;

/// Read `OscPacket`s from udp server until no more messages are received and then dispatch them
fn receive_packets(mut disp: ResMut<OscDispatcher>, osc_server: Res<OscUdpServer>, method_query: Query<&mut OscMethod>, multi_method_query: Query<&mut OscMultiMethod>) {
    let mut osc_packets = vec![];

    loop {
        match osc_server.recv() {
            Ok(o) => match o {
                Some(p) => osc_packets.push(p),
                None => break
            },
            Err(_) => ()
        }
    }

    disp.dispatch(osc_packets, method_query, multi_method_query);
}

pub struct OscClients {
    pub clients: Vec<OscUdpClient>,
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)

        .insert_resource(OscUdpServer::new("0.0.0.0:31337").unwrap())
        .insert_resource(OscDispatcher::default())
        .insert_resource(OscClients {
            clients: vec![
                // TouchOSC
                OscUdpClient::new(SocketAddrV4::new(Ipv4Addr::from([192, 168, 42, 129]), 31337).into()).unwrap(),
                // Pyree Engine
                OscUdpClient::new(SocketAddrV4::new(Ipv4Addr::from([127, 0, 0, 1]), 31338).into()).unwrap(),
                // Loopback
                OscUdpClient::new(SocketAddrV4::new(Ipv4Addr::from([127, 0, 0, 1]), 31337).into()).unwrap(),
            ]
        })
        .add_system(receive_packets)

        .add_system_set(pyree_system_set())
        .add_startup_system_set(pyree_startup_system_set())

        .run();
}
