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
}

impl RandomValComponent {
    pub fn new(index: u32) -> Self {
        Self{
            index,
            value: 0.0
        }
    }

    fn beat(&mut self, osc_client: &OscUdpClient) {
        self.set_val(osc_client, random());
    }

    fn rotary(&mut self, osc_client: &OscUdpClient, osc_message: OscMessage) {
        if osc_message.args.len() == 1 {
            match osc_message.args[0] {
                OscType::Float(val) => self.set_val(osc_client, val),
                _ => {}
            }
        }
    }

    fn set_val(&mut self, osc_client: &OscUdpClient, value: f32) {
        self.value = value;

        let out: Vec<OscMessage> = vec![
            OscMessage { addr: format!("/randomval/label{}", self.index), args: vec![format!("{:.2}", self.value).into()] },
            OscMessage { addr: format!("/randomval/rotary{}", self.index), args: vec![self.value.into()] },
        ];

        for msg in out {
            match osc_client.send(&OscPacket::Message(msg)) {
                Ok(_) => (),
                Err(e) => println!("{}", e)
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
    osc_multi_method: OscMultiMethod
}

impl RandomValBundle {
    pub fn new(index: u32) -> Self {
        Self {
            random_val: RandomValComponent::new(index),
            osc_multi_method: OscMultiMethod{
                methods: RandomValComponent::gen_osc_methods(index)
            }
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
            rvc.beat(osc_client.deref_mut())
        }
        // Rotary
        if let Some(msg) = get_newest_message(&mut omm.methods[1]) {
            rvc.rotary(osc_client.deref_mut(), msg)
        }
    }
}