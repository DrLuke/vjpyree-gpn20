use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy_osc::{OscMethod, OscUdpClient};
use rosc::{OscMessage, OscPacket, OscType};
use crate::OscClients;

#[derive(Component)]
pub struct BeatForwarder;

#[derive(Component)]
pub struct BeatMuted(pub bool);

#[derive(Component)]
pub struct BeatMuter;

pub fn spawn_beat_forwarder(mut commands: Commands) {
    commands.spawn_bundle((
        BeatForwarder {},
        OscMethod::new("/beat").unwrap()
    ));
    commands.spawn_bundle((
        BeatMuter,
        OscMethod::new("/beat/mute").unwrap()
    ));
    commands.insert_resource(BeatMuted(false));
}

/// Takes any received beat message and forwards it to the visuals engine
pub fn beat_fwd_system(osc_clients: ResMut<OscClients>, mut query: Query<(&BeatForwarder, &mut OscMethod), Changed<OscMethod>>, beat_muted: ResMut<BeatMuted>) {
    let (_, mut osc_method) = match query.get_single_mut() {
        Ok(r) => r,
        Err(e) => return
    };
    if let Some(msg) = osc_method.get_message() {
        if !beat_muted.0 {
            if let Err(e) = osc_clients.clients[1].send(&OscPacket::Message(msg)) {
                println!("Error sending beat: {}", e);
            }
        }
    }
}

pub fn beat_mute_system(osc_clients: ResMut<OscClients>, mut query: Query<(&BeatMuter, &mut OscMethod), Changed<OscMethod>>, mut beat_muted: ResMut<BeatMuted>) {
    let (_, mut osc_method) = match query.get_single_mut() {
        Ok(r) => r,
        Err(e) => return
    };
    if let Some(msg) = osc_method.get_message() {
        if msg.args.len() == 1 {
            if let OscType::Float(val) = msg.args[0] {
                beat_muted.0 = (val > 0.);
                osc_clients.clients[0].send(&OscPacket::Message(OscMessage { addr: "/beat/mute".to_string(), args: vec![val.into()] })).unwrap_or(());
            }
        }
    }
}