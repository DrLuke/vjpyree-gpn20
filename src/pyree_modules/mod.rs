use bevy::prelude::*;

mod randomval;

use randomval::random_val_receive;
use crate::pyree_modules::randomval::{RandomValBundle};

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

pub fn pyree_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(random_val_receive)
}

pub fn pyree_startup_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(spawn_randomval)
}