use std::thread::sleep;
use std::time::Duration;
use bevy::prelude::*;
use bevy_osc::OscUdpClient;

mod randomval;
mod misc;
mod toggle;
mod button;

use randomval::random_val_receive;
use crate::OscClients;
use crate::pyree_modules::misc::{beat_fwd_system, spawn_beat_forwarder};
use crate::pyree_modules::randomval::{init_randomval_gui_system, RandomValBundle, RandomValComponent};
use crate::pyree_modules::toggle::{init_toggle_gui_system, toggle_system_receive, ToggleBundle};
use crate::pyree_modules::button::{button_system_receive, ButtonBundle};

fn spawn_randomval(mut commands: Commands) {
    commands.spawn_bundle(RandomValBundle::new(0, "test".to_string()));
    commands.spawn_bundle(RandomValBundle::new(1, "test".to_string()));
    commands.spawn_bundle(RandomValBundle::new(2, "test".to_string()));
    commands.spawn_bundle(RandomValBundle::new(3, "test".to_string()));
    commands.spawn_bundle(RandomValBundle::new(4, "test".to_string()));
    commands.spawn_bundle(RandomValBundle::new(5, "test".to_string()));
    commands.spawn_bundle(RandomValBundle::new(6, "test".to_string()));
    commands.spawn_bundle(RandomValBundle::new(7, "test".to_string()));
    commands.spawn_bundle(RandomValBundle::new(8, "test".to_string()));
    commands.spawn_bundle(RandomValBundle::new(9, "test".to_string()));
}

fn spawn_toggle(mut commands: Commands) {
    // Reaction diffusion
    // * Pattern select
    for i in 0..5 {
        commands.spawn_bundle(ToggleBundle::new(i, "rdpat".to_string(), i));
    }

    /*for i in 0..5 {
        commands.spawn_bundle(ToggleBundle::new(3+i, "testmulti".to_string(), i));
    }*/
}

fn spawn_button(mut commands: Commands) {
    commands.spawn_bundle(ButtonBundle::simple(0));
}


pub fn pyree_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(random_val_receive)
        .with_system(toggle_system_receive)
        .with_system(button_system_receive)
        .with_system(init_randomval_gui_system)
        .with_system(init_toggle_gui_system)
        .with_system(beat_fwd_system)
}

pub fn pyree_startup_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(spawn_randomval)
        .with_system(spawn_toggle)
        .with_system(spawn_beat_forwarder)
        .with_system(spawn_button)
}