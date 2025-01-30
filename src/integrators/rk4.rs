use crate::models::State;
use crate::physics::gravity::gravity_acceleration;

pub fn integrate(state: &State, dt: f64) -> State {
    let k1_v = gravity_acceleration(&state.position);
    let k1_r = state.velocity;

    let k2_pos = state.position + (k1_r * dt / 2.0);
    let k2_v = gravity_acceleration(&k2_pos);
    let k2_r = state.velocity + (k1_v * dt / 2.0);

    let k3_pos = state.position + (k2_r * dt / 2.0);
    let k3_v = gravity_acceleration(&k3_pos);
    let k3_r = state.velocity + (k2_v * dt / 2.0);

    let k4_pos = state.position + (k3_r * dt);
    let k4_v = gravity_acceleration(&k4_pos);
    let k4_r = state.velocity + (k3_v * dt);

    let new_position = state.position + 
        (k1_r + k2_r * 2.0 + k3_r * 2.0 + k4_r) * (dt / 6.0);
    let new_velocity = state.velocity + 
        (k1_v + k2_v * 2.0 + k3_v * 2.0 + k4_v) * (dt / 6.0);

    State::new(state.mass, new_position, new_velocity)
} 