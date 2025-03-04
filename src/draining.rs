use bevy::prelude::*;

use crate::water::Water;

const DRAIN_RATE: f32 = 2.4;

pub struct DrainingPlugin;

impl Plugin for DrainingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, drain_water);
    }
}

fn drain_water(
    mut waters: Query<(&mut Water, &mut Transform)>, 
    time: Res<Time>
) {
    let delta_time = time.delta_secs();

    for (mut water, mut transform) in waters.iter_mut() {
        let drain_amount = DRAIN_RATE * delta_time;
        water.amount -= drain_amount;
        transform.translation.y -= drain_amount;
    }
}
