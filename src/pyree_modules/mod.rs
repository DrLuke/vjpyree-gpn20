use bevy::prelude::*;
use bevy_osc::OscMultiMethod;

mod randomval;

use randomval::random_val_receive;
use crate::pyree_modules::randomval::{RandomVal, RandomValBundle, RandomValComponent};

fn spawn_randomval(mut commands: Commands) {
    commands.spawn_bundle(RandomValBundle::new(0));
}

pub fn pyree_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(random_val_receive)
}

pub fn pyree_startup_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(spawn_randomval)
}