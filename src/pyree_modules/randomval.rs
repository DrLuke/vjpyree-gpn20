use std::ops::DerefMut;
use bevy::prelude::*;
use bevy_osc::{OscMethod, OscMultiMethod, OscUdpClient};
use rand::prelude::random;
use rosc::{OscMessage, OscPacket, OscType};

/// Generates a random value on every beat
#[derive(Component)]
pub struct RandomValComponent {
    index: u32,
    value: f32,
    label: String,
}

impl RandomValComponent {
    pub fn new(index: u32, label: String) -> Self {
        Self {
            index,
            value: 0.0,
            label,
        }
    }

    fn num_label_msg(&self) -> OscMessage { OscMessage { addr: format!("/randomval/numlabel{}", self.index), args: vec![format!("{:.2}", self.value).into()] } }
    fn rotary_msg(&self) -> OscMessage { OscMessage { addr: format!("/randomval/rotary{}", self.index), args: vec![self.value.into()] } }

    fn on_beat(&mut self, osc_client: &OscUdpClient) {
        self.value = random();

        self.send_messages(osc_client, vec![
            self.num_label_msg(),
            self.rotary_msg(),
        ])
    }

    fn on_rotary(&mut self, osc_client: &OscUdpClient, osc_message: OscMessage) {
        if osc_message.args.len() == 1 {
            if let OscType::Float(val) = osc_message.args[0] {
                self.value = val;

                self.send_messages(osc_client, vec![
                    self.num_label_msg(),
                ])
            }
        }
    }

    fn send_messages(&self, osc_client: &OscUdpClient, messages: Vec<OscMessage>) {
        for msg in messages {
            if let Err(e) = osc_client.send(&OscPacket::Message(msg)) {
                println!("{}", e);
            }
        }
    }

    pub fn gen_osc_methods(index: u32) -> Vec<OscMethod> {
        vec![
            OscMethod::new("/beat").unwrap(),
            OscMethod::new(format!("/randomval/rotary{}", index).as_str()).unwrap(),
            OscMethod::new(format!("/randomval/toggle{}/1/1", index).as_str()).unwrap(),
            OscMethod::new(format!("/randomval/toggle{}/1/2", index).as_str()).unwrap(),
            OscMethod::new(format!("/randomval/toggle{}/1/3", index).as_str()).unwrap(),
            OscMethod::new(format!("/randomval/toggle{}/1/4", index).as_str()).unwrap(),
            OscMethod::new(format!("/randomval/toggle{}/1/5", index).as_str()).unwrap(),
        ]
    }
}

#[derive(Bundle)]
#[derive(Component)]
pub struct RandomValBundle {
    random_val: RandomValComponent,
    osc_multi_method: OscMultiMethod,
}

impl RandomValBundle {
    pub fn new(index: u32, label: String) -> Self {
        Self {
            random_val: RandomValComponent::new(index, label),
            osc_multi_method: OscMultiMethod {
                methods: RandomValComponent::gen_osc_methods(index)
            },
        }
    }
}

/// Discard all but the newest message
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

/// Receive OSC messages
pub fn random_val_receive(mut osc_client: ResMut<OscUdpClient>, mut query: Query<(&mut RandomValComponent, &mut OscMultiMethod), Changed<OscMultiMethod>>) {
    for (mut rvc, mut omm) in query.iter_mut() {
        // Beat
        if let Some(_) = get_newest_message(&mut omm.methods[0]) {
            rvc.on_beat(osc_client.deref_mut())
        }
        // Rotary
        if let Some(msg) = get_newest_message(&mut omm.methods[1]) {
            rvc.on_rotary(osc_client.deref_mut(), msg)
        }
    }
}