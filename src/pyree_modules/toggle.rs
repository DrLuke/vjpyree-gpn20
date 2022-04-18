use bevy::prelude::*;
use bevy_osc::{OscMethod, OscUdpClient};
use rosc::{OscMessage, OscPacket, OscType};
use crate::OscClients;

#[derive(Component)]
pub struct ToggleComponent {
    index: u32,
    value: f32,
}

pub struct ToggleBundle {
    toggle_component: ToggleComponent,
    osc_method: OscMethod,
}

impl ToggleBundle {
    pub fn new(index: u32) -> Self {
        ToggleBundle {
            toggle_component: ToggleComponent {
                index,
                value: 0.0,
            },
            osc_method: OscMethod::new(format!("/toggle/{}", index).as_str()).unwrap(),
        }
    }
}

impl ToggleComponent {
    fn engine_msg(&self) -> OscMessage { OscMessage { addr: format!("/toggle/{}", self.index), args: vec![self.value.into()] } }
    fn ui_msg(&self) -> OscMessage { self.engine_msg() }

    /// osc client is pyree client
    pub fn update_val(&mut self, val: f32, osc_client: &OscUdpClient) {
        self.value = val;
        osc_client.send(&OscPacket::Message(self.engine_msg())).unwrap_or(());
    }

    pub fn update_ui(&self, osc_client: &OscUdpClient) {
        osc_client.send(&OscPacket::Message(self.engine_msg())).unwrap_or(());
    }
}

fn get_newest_message(osc_method: &mut OscMethod) -> Option<OscMessage> {
    let mut messages: Vec<OscMessage> = vec![];
    loop {
        match osc_method.get_message() {
            Some(msg) => messages.push(msg),
            None => break
        }
    }
    messages.pop()
}

pub fn toggle_system(osc_clients: ResMut<OscClients>, mut query: Query<(&mut ToggleComponent, &mut OscMethod), Changed<OscMethod>>) {
    for (mut tc, mut om) in query.iter_mut() {
        if let Some(msg) = get_newest_message(&mut om) {
            if msg.args.len() == 1 {
                if let OscType::Float(val) = msg.args[0] {
                    tc.update_val(val, &osc_clients.clients[1])
                }
            }
        }
    }
}
