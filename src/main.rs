use std::net::{Ipv4Addr, SocketAddrV4};
use bevy::prelude::*;
use bevy_osc::{OscDispatcher, OscMethod, OscMultiMethod, OscUdpClient, OscUdpServer};

/// Read `OscPacket`s from udp server until no more messages are received and then dispatch them
fn receive_packets(mut disp: ResMut<OscDispatcher>, mut query: Query<&mut OscUdpServer>, method_query: Query<&mut OscMethod>, multi_method_query: Query<&mut OscMultiMethod>) {
    let mut osc_udp_server = query.single_mut();
    let mut osc_packets = vec![];

    loop {
        match osc_udp_server.recv() {
            Ok(o) => match o {
                Some(p) => osc_packets.push(p),
                None => break
            },
            Err(_) => ()
        }
    }

    disp.dispatch(osc_packets, method_query, multi_method_query);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        .insert_resource(OscUdpServer::new("0.0.0.0:31337"))
        .insert_resource(OscDispatcher::default())
        .insert_resource(OscUdpClient::new(SocketAddrV4::new(Ipv4Addr::from([1,2,3,4]), 31337).into()))
        .add_system(receive_packets)

        .run();
}
