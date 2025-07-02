use bevy::prelude::*;
use crate::components::SpinningCube;

pub fn rotate_cube(mut query: Query<&mut Transform, With<SpinningCube>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() * 1.0);
    }
}
