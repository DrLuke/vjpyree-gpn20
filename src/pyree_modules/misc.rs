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

#[derive(Component)]
pub struct Traktor {
    pub count: i32,
}

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
    commands.spawn_bundle((
        Traktor { count: 0 },
        OscMethod::new("/traktor/beat").unwrap()
    ));
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

pub fn traktor_system(osc_clients: ResMut<OscClients>, mut query: Query<(&mut Traktor, &mut OscMethod), Changed<OscMethod>>, mut beat_muted: ResMut<BeatMuted>) {
    let (mut traktor, mut osc_method) = match query.get_single_mut() {
        Ok(r) => r,
        Err(e) => return
    };
    if let Some(msg) = osc_method.get_message() {
        if beat_muted.0 { return; }
        // Messaged from OSC push
        if msg.args.len() == 1 {
            if let OscType::Float(val) = msg.args[0] {
                if val < 0. {
                    traktor.count -= 1;
                } else {
                    traktor.count += 1;
                }
            }
        } else if msg.args.len() == 0 {
            // Else if the message comes from traktor script directly it might no have a value, just increment counter
            traktor.count += 1;
        }

        // Count up to 24, send beat on 0, reset beat indicator on 12
        traktor.count = traktor.count.rem_euclid(24);
        if traktor.count == 0 {
            osc_clients.clients[0].send(&OscPacket::Message(OscMessage { addr: "/beat".to_string(), args: vec![] })).unwrap_or(());
            osc_clients.clients[0].send(&OscPacket::Message(OscMessage { addr: "/beat/led".to_string(), args: vec![(0.1).into()] })).unwrap_or(());
        }
        if traktor.count == 12 {
            osc_clients.clients[0].send(&OscPacket::Message(OscMessage { addr: "/beat/led".to_string(), args: vec![(0.0).into()] })).unwrap_or(());
        }
    }
}