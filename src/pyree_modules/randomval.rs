use std::ops::DerefMut;
use bevy::prelude::*;
use bevy_osc::{OscMethod, OscMultiMethod, OscUdpClient};
use rand::prelude::random;
use rosc::{OscBundle, OscMessage, OscPacket, OscTime, OscType};
use crate::OscClients;
use crate::pyree_modules::misc::BeatMuted;

/// Generates a random value on every beat
#[derive(Component)]
pub struct RandomValComponent {
    index: u32,
    // Horizontal index
    value: f32,
    // Value of randval
    label: String,
    // Label that show up above randval
    on_beat: bool,
    // Trigger on every n-th beat
    beat_counter: u32,
    beat_divisor: u32,
    // Max change of value on beat
    delta: f32,
    // Wrap value if it crosses 0,1 range
    wrap: bool,
    multi_index: u32,
    // Offset for multipush index yadda yadda
    multi_prefix: String,   // actually a postfix that is added to the multi-input names etc. but I was too lazy to refactor the field name
}

impl RandomValComponent {
    pub fn new(index: u32, label: String, multi_index: u32, multi_prefix: String) -> Self {
        Self {
            index,
            value: 0.0,
            label,
            on_beat: false,
            beat_counter: 0,
            beat_divisor: 1,
            delta: 1.,
            wrap: true,
            multi_index,
            multi_prefix,
        }
    }

    fn label_msg(&self) -> OscMessage { OscMessage { addr: format!("/randomval/label{}", self.index), args: vec![self.label.as_str().into()] } }
    fn num_label_msg(&self) -> OscMessage { OscMessage { addr: format!("/randomval/numlabel{}", self.index), args: vec![format!("{:.2}", self.value).into()] } }
    fn rotary_msg(&self) -> OscMessage { OscMessage { addr: format!("/randomval/rotary{}", self.index), args: vec![self.value.into()] } }
    fn colors_msg(&self) -> OscMessage {
        if self.on_beat {
            OscMessage { addr: format!("/randomval/numlabel{}/color", self.index), args: vec!["orange".into()] }
        } else {
            OscMessage { addr: format!("/randomval/numlabel{}/color", self.index), args: vec!["red".into()] }
        }
    }
    fn div_label_msg(&self) -> OscMessage { OscMessage { addr: format!("/randomval/div_label{}", self.index), args: vec![format!("{: >3}  :  {: <3}", self.beat_counter + 1, self.beat_divisor).into()] } }
    fn on_beat_msg(&self) -> OscMessage { OscMessage { addr: format!("/randomval/on_beat{}/1/{}", self.multi_prefix, self.index + 1 - self.multi_index), args: vec![(if self.on_beat { 1 } else { 0 }).into()] } }
    fn wrap_msg(&self) -> OscMessage { OscMessage { addr: format!("/randomval/wrap{}", self.index), args: vec![(if self.on_beat { 1 } else { 0 }).into()] } }
    fn delta_msg(&self) -> OscMessage { OscMessage { addr: format!("/randomval/delta{}", self.index), args: vec![self.delta.into()] } }
    fn engine_msg(&self) -> OscMessage { OscMessage { addr: format!("/randomval/{}", self.index), args: vec![self.value.into()] } }

    fn on_beat(&mut self, osc_client: &OscUdpClient, engine_client: &OscUdpClient) {
        if !self.on_beat { return; }
        self.beat(osc_client, engine_client);
    }

    fn on_rotary(&mut self, osc_client: &OscUdpClient, engine_client: &OscUdpClient, osc_message: OscMessage) {
        if osc_message.args.len() == 1 {
            if let OscType::Float(val) = osc_message.args[0] {
                self.value = val;

                self.send_messages(osc_client, vec![
                    self.num_label_msg(),
                ]);
                self.send_messages(engine_client, vec![
                    self.engine_msg(),
                ]);
            }
        }
    }

    fn on_set_on_beat(&mut self, osc_client: &OscUdpClient, osc_message: OscMessage) {
        if osc_message.args.len() == 1 {
            if let OscType::Float(val) = osc_message.args[0] {
                self.on_beat = if val > 0. { true } else { false };
                self.send_messages(osc_client, vec![self.colors_msg()])
            }
        }
    }

    fn on_inc_div(&mut self, osc_client: &OscUdpClient, osc_message: OscMessage) {
        if osc_message.args.len() == 1 {
            if let OscType::Float(val) = osc_message.args[0] {
                if val > 0. {
                    self.beat_divisor += 1;
                    self.send_messages(osc_client, vec![self.   div_label_msg()])
                }
            }
        }
    }

    fn on_dec_div(&mut self, osc_client: &OscUdpClient, osc_message: OscMessage) {
        if osc_message.args.len() == 1 {
            if let OscType::Float(val) = osc_message.args[0] {
                if val > 0. {
                    if self.beat_divisor > 1 {
                        self.beat_divisor -= 1;
                        self.send_messages(osc_client, vec![self.div_label_msg()])
                    }
                }
            }
        }
    }

    fn on_trig_beat(&mut self, osc_client: &OscUdpClient, engine_client: &OscUdpClient, osc_message: OscMessage) {
        if osc_message.args.len() == 1 {
            if let OscType::Float(val) = osc_message.args[0] {
                if val > 0. {
                    self.beat(osc_client, engine_client)
                }
            }
        }
    }

    fn on_delta(&mut self, osc_client: &OscUdpClient, osc_message: OscMessage) {
        if osc_message.args.len() == 1 {
            if let OscType::Float(val) = osc_message.args[0] {
                self.delta = val;
            }
        }
    }

    fn on_wrap(&mut self, osc_client: &OscUdpClient, osc_message: OscMessage) {
        if osc_message.args.len() == 1 {
            if let OscType::Float(val) = osc_message.args[0] {
                self.wrap = if val > 0. { true } else { false };
            }
        }
    }

    fn beat(&mut self, osc_client: &OscUdpClient, engine_client: &OscUdpClient) {
        self.beat_counter = (self.beat_counter + 1) % self.beat_divisor;
        if self.beat_counter == 0 {
            self.value = (self.value + (random::<f32>() - 0.5) * self.delta);
            if self.wrap {
                self.value = self.value.rem_euclid(1.0);
            } else {
                self.value = self.value.min(1.).max(0.);
            }
        }

        self.send_messages(osc_client, vec![
            //self.num_label_msg(),
            self.rotary_msg(),
            //self.div_label_msg(),
        ]);
        self.send_messages(engine_client, vec![
            self.engine_msg()
        ]);
    }

    pub fn update_ui(&mut self, osc_client: &OscUdpClient) {
        let bundle = OscBundle {
            timetag: OscTime { seconds: 0, fractional: 0 },
            content: vec![
                OscPacket::Message(self.label_msg()),
                OscPacket::Message(self.num_label_msg()),
                OscPacket::Message(self.rotary_msg()),
                OscPacket::Message(self.colors_msg()),
                OscPacket::Message(self.div_label_msg()),
                OscPacket::Message(self.on_beat_msg()),
                OscPacket::Message(self.wrap_msg()),
                OscPacket::Message(self.delta_msg()),
            ],
        };
        if let Err(e) = osc_client.send(&OscPacket::Bundle(bundle)) {
            println!("{}", e);
        }
    }

    fn send_messages(&self, osc_client: &OscUdpClient, messages: Vec<OscMessage>) {
        for msg in messages {
            if let Err(e) = osc_client.send(&OscPacket::Message(msg)) {
                println!("{}", e);
            }
        }
    }

    pub fn gen_osc_methods(&self, index: u32) -> Vec<OscMethod> {
        vec![
            OscMethod::new("/beat").unwrap(),
            OscMethod::new(format!("/randomval/rotary{}", index).as_str()).unwrap(),
            OscMethod::new(format!("/randomval/on_beat{}/1/{}", self.multi_prefix, index + 1 - self.multi_index).as_str()).unwrap(),
            OscMethod::new(format!("/randomval/div{}/3/{}", self.multi_prefix, index + 1 - self.multi_index).as_str()).unwrap(),
            OscMethod::new(format!("/randomval/div{}/2/{}", self.multi_prefix, index + 1 - self.multi_index).as_str()).unwrap(),
            OscMethod::new(format!("/randomval/div{}/1/{}", self.multi_prefix, index + 1 - self.multi_index).as_str()).unwrap(),
            OscMethod::new(format!("/randomval/delta{}", index).as_str()).unwrap(),
            OscMethod::new(format!("/randomval/wrap{}", index).as_str()).unwrap(),
            OscMethod::new("/updateui").unwrap(),
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
    pub fn new(index: u32, label: String, multi_index: u32, multi_prefix: String) -> Self {
        let rvc = RandomValComponent::new(index, label, multi_index, multi_prefix);
        let methods = rvc.gen_osc_methods(index);
        Self {
            random_val: rvc,
            osc_multi_method: OscMultiMethod {
                methods
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
pub fn random_val_receive(mut osc_clients: ResMut<OscClients>, mut query: Query<(&mut RandomValComponent, &mut OscMultiMethod), Changed<OscMultiMethod>>, beat_muted: ResMut<BeatMuted>) {
    let mut touch_osc_client = &osc_clients.clients[0];
    let mut engine_osc_client = &osc_clients.clients[1];

    for (mut rvc, mut omm) in query.iter_mut() {
        // Beat
        if let Some(_) = get_newest_message(&mut omm.methods[0]) {
            if !beat_muted.0 {
                rvc.on_beat(touch_osc_client, engine_osc_client)
            }
        }
        // Rotary
        if let Some(msg) = get_newest_message(&mut omm.methods[1]) {
            rvc.on_rotary(touch_osc_client, engine_osc_client, msg)
        }
        // OnBeat toggle
        if let Some(msg) = get_newest_message(&mut omm.methods[2]) {
            rvc.on_set_on_beat(touch_osc_client, msg)
        }
        // Increment divisor
        if let Some(msg) = get_newest_message(&mut omm.methods[3]) {
            rvc.on_inc_div(touch_osc_client, msg)
        }
        // Trigger beat
        if let Some(msg) = get_newest_message(&mut omm.methods[4]) {
            rvc.on_trig_beat(touch_osc_client, engine_osc_client, msg)
        }
        // Decrement divisor
        if let Some(msg) = get_newest_message(&mut omm.methods[5]) {
            rvc.on_dec_div(touch_osc_client, msg)
        }
        // Delta rotary
        if let Some(msg) = get_newest_message(&mut omm.methods[6]) {
            rvc.on_delta(touch_osc_client, msg)
        }
        // OnBeat toggle
        if let Some(msg) = get_newest_message(&mut omm.methods[7]) {
            rvc.on_wrap(touch_osc_client, msg)
        }
        // Update UI
        if let Some(msg) = get_newest_message(&mut omm.methods[8]) {
            rvc.update_ui(touch_osc_client);
        }
    }
}

pub fn init_randomval_gui_system(osc_client: ResMut<OscClients>, mut query: Query<&mut RandomValComponent, Added<RandomValComponent>>) {
    for mut rvc in query.iter_mut() {
        rvc.update_ui(&osc_client.clients[0]);
    }
}