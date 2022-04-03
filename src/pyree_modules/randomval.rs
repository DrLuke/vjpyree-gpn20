use std::ops::DerefMut;
use bevy::prelude::*;
use bevy_osc::{OscMethod, OscMultiMethod, OscUdpClient};
use rand::prelude::random;
use rosc::{OscMessage, OscPacket, OscType};

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
    wrap: bool
}

impl RandomValComponent {
    pub fn new(index: u32, label: String) -> Self {
        Self {
            index,
            value: 0.0,
            label,
            on_beat: false,
            beat_counter: 0,
            beat_divisor: 1,
            delta: 1.,
            wrap: true
        }
    }

    fn num_label_msg(&self) -> OscMessage { OscMessage { addr: format!("/randomval/numlabel{}", self.index), args: vec![format!("{:.2}", self.value).into()] } }
    fn rotary_msg(&self) -> OscMessage { OscMessage { addr: format!("/randomval/rotary{}", self.index), args: vec![self.value.into()] } }
    fn colors_msg(&self, enabled: bool) -> OscMessage {
        if enabled {
            OscMessage { addr: format!("/randomval/numlabel{}/color", self.index), args: vec!["orange".into()] }
        } else {
            OscMessage { addr: format!("/randomval/numlabel{}/color", self.index), args: vec!["red".into()] }
        }
    }
    fn div_label_msg(&self) -> OscMessage { OscMessage { addr: format!("/randomval/div_label{}", self.index), args: vec![format!("{: >3}  :  {: <3}", self.beat_counter + 1, self.beat_divisor).into()] } }

    fn on_beat(&mut self, osc_client: &OscUdpClient) {
        if !self.on_beat { return; }
        self.beat(osc_client);
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

    fn on_set_on_beat(&mut self, osc_client: &OscUdpClient, osc_message: OscMessage) {
        if osc_message.args.len() == 1 {
            if let OscType::Float(val) = osc_message.args[0] {
                self.on_beat = if val > 0. { true } else { false };
                self.send_messages(osc_client, vec![self.colors_msg(self.on_beat)])
            }
        }
    }

    fn on_inc_div(&mut self, osc_client: &OscUdpClient, osc_message: OscMessage) {
        if osc_message.args.len() == 1 {
            if let OscType::Float(val) = osc_message.args[0] {
                if val > 0. {
                    self.beat_divisor += 1;
                    self.send_messages(osc_client, vec![self.div_label_msg()])
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

    fn on_trig_beat(&mut self, osc_client: &OscUdpClient, osc_message: OscMessage) {
        if osc_message.args.len() == 1 {
            if let OscType::Float(val) = osc_message.args[0] {
                if val > 0. {
                    self.beat(osc_client)
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

    fn beat(&mut self, osc_client: &OscUdpClient) {
        self.beat_counter = (self.beat_counter + 1) % self.beat_divisor;
        if self.beat_counter == 0 {
            self.value = (self.value + (random::<f32>()-0.5) * self.delta);
            if self.wrap {
                self.value = self.value.rem_euclid(1.0);
            } else {
                self.value = self.value.min(1.).max(0.);
            }
        }

        self.send_messages(osc_client, vec![
            self.num_label_msg(),
            self.rotary_msg(),
            self.div_label_msg(),
        ])
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
            OscMethod::new(format!("/randomval/on_beat/1/{}", index + 1).as_str()).unwrap(),
            OscMethod::new(format!("/randomval/div/3/{}", index + 1).as_str()).unwrap(),
            OscMethod::new(format!("/randomval/div/2/{}", index + 1).as_str()).unwrap(),
            OscMethod::new(format!("/randomval/div/1/{}", index + 1).as_str()).unwrap(),
            OscMethod::new(format!("/randomval/delta{}", index).as_str()).unwrap(),
            OscMethod::new(format!("/randomval/wrap{}", index).as_str()).unwrap(),
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
        // OnBeat toggle
        if let Some(msg) = get_newest_message(&mut omm.methods[2]) {
            rvc.on_set_on_beat(osc_client.deref_mut(), msg)
        }
        // Increment divisor
        if let Some(msg) = get_newest_message(&mut omm.methods[3]) {
            rvc.on_inc_div(osc_client.deref_mut(), msg)
        }
        // Trigger beat
        if let Some(msg) = get_newest_message(&mut omm.methods[4]) {
            rvc.on_trig_beat(osc_client.deref_mut(), msg)
        }
        // Decrement divisor
        if let Some(msg) = get_newest_message(&mut omm.methods[5]) {
            rvc.on_dec_div(osc_client.deref_mut(), msg)
        }
        // Delta rotary
        if let Some(msg) = get_newest_message(&mut omm.methods[6]) {
            rvc.on_delta(osc_client.deref_mut(), msg)
        }
        // OnBeat toggle
        if let Some(msg) = get_newest_message(&mut omm.methods[7]) {
            rvc.on_wrap(osc_client.deref_mut(), msg)
        }
    }
}