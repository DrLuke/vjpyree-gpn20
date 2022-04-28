use bevy::prelude::*;
use bevy_osc::{OscMethod, OscUdpClient};
use rosc::{OscMessage, OscPacket, OscType};
use crate::OscClients;

#[derive(Component)]
pub struct ButtonComponent {
    index: u32,
    value: f32,
    button_prefix: String,
    multi_index: u32,
}

#[derive(Bundle)]
#[derive(Component)]
pub struct ButtonBundle {
    button_component: ButtonComponent,
    osc_method: OscMethod,
}

impl ButtonBundle {
    pub fn simple(index: u32) -> Self { Self::new(index, "button".to_string(), 0) }

    pub fn new(index: u32, button_prefix: String, multi_index: u32) -> Self {
        ButtonBundle {
            button_component: ButtonComponent {
                index,
                value: 0.0,
                button_prefix: button_prefix.clone(),
                multi_index,
            },
            osc_method: {
                let method;
                if button_prefix == "button" {
                    // For single button
                    method = OscMethod::new(format!("/button/{}", index).as_str()).unwrap();
                } else {
                    // For horizontal multibutton
                    method = OscMethod::new(format!("/{}/1/{}", button_prefix, multi_index + 1).as_str()).unwrap();
                }
                method
            },
        }
    }
}

impl ButtonComponent {
    fn engine_msg(&self) -> OscMessage { OscMessage { addr: format!("/button/{}", self.index), args: vec![self.value.into()] } }

    /// osc client is pyree client
    pub fn update_val(&mut self, val: f32, osc_client: &OscUdpClient) {
        self.value = val;
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

pub fn button_system_receive(osc_clients: ResMut<OscClients>, mut query: Query<(&mut ButtonComponent, &mut OscMethod), Changed<OscMethod>>) {
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
