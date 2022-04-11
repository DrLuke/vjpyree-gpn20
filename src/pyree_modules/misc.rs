use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy_osc::OscMethod;
use rosc::OscPacket;
use crate::OscClients;

#[derive(Component)]
pub struct BeatForwarder;

pub fn spawn_beat_forwarder(mut commands: Commands) {
    commands.spawn_bundle((
        BeatForwarder {},
        OscMethod::new("/beat").unwrap()
    ));
}

/// Takes any received beat message and forwards it to the visuals engine
pub fn beat_fwd_system(osc_clients: ResMut<OscClients>, mut query: Query<(&BeatForwarder, &mut OscMethod), Changed<OscMethod>>) {
    let (_, mut osc_method) = match query.get_single_mut() {
        Ok(r) => r,
        Err(e) => return
    };
    if let Some(msg) = osc_method.get_message() {
        if let Err(e) = osc_clients.clients[1].send(&OscPacket::Message(msg)) {
            println!("Error sending beat: {}", e);
        }
    }
}