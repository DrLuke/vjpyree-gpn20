use bevy::prelude::*;
use bevy_osc::{OscMethod, OscUdpClient};
use rosc::{OscMessage, OscPacket, OscType};
use crate::OscClients;

#[derive(Component)]
pub struct ToggleComponent {
    index: u32,
    value: f32,
    toggle_prefix: String,
    multi_index: u32,
}

#[derive(Bundle)]
#[derive(Component)]
pub struct ToggleBundle {
    toggle_component: ToggleComponent,
    osc_method: OscMethod,
}

impl ToggleBundle {
    pub fn simple(index: u32) -> Self { Self::new(index, "toggle".to_string(), 0) }

    pub fn new(index: u32, toggle_prefix: String, multi_index: u32) -> Self {
        ToggleBundle {
            toggle_component: ToggleComponent {
                index,
                value: 0.0,
                toggle_prefix: toggle_prefix.clone(),
                multi_index,
            },
            osc_method: {
                let method;
                if toggle_prefix == "toggle" {
                    // For single toggle
                    method = OscMethod::new(format!("/toggle/{}", index).as_str()).unwrap();
                } else {
                    // For horizontal multitoggles
                    method = OscMethod::new(format!("/{}/1/{}", toggle_prefix, multi_index + 1).as_str()).unwrap();
                }
                method
            },
        }
    }
}

impl ToggleComponent {
    fn engine_msg(&self) -> OscMessage { OscMessage { addr: format!("/toggle/{}", self.index), args: vec![self.value.into()] } }
    fn ui_msg(&self) -> OscMessage {
        return if self.toggle_prefix == "toggle" {
            self.engine_msg()
        } else {
            OscMessage { addr: format!("/{}/1/{}", self.toggle_prefix, self.multi_index + 1), args: vec![self.value.into()] }
        };
    }

    /// osc client is pyree client
    pub fn update_val(&mut self, val: f32, osc_client: &OscUdpClient) {
        self.value = val;
        osc_client.send(&OscPacket::Message(self.engine_msg())).unwrap_or(());
    }

    pub fn update_ui(&self, osc_client: &OscUdpClient) {
        osc_client.send(&OscPacket::Message(self.ui_msg())).unwrap_or(());
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

pub fn toggle_system_receive(osc_clients: ResMut<OscClients>, mut query: Query<(&mut ToggleComponent, &mut OscMethod), Changed<OscMethod>>) {
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

pub fn init_toggle_gui_system(osc_client: ResMut<OscClients>, mut query: Query<&mut ToggleComponent, Added<ToggleComponent>>) {
    for mut tc in query.iter_mut() {
        tc.update_ui(&osc_client.clients[0]);
    }
}
