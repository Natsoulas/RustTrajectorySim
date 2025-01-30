mod constants;
mod models;
mod physics;
mod integrators;

use constants::*;
use models::State;
use physics::energy::{calculate_energy, calculate_angular_momentum};
use nalgebra as na;

fn main() {
    let orbital_radius = R_EARTH + ORBIT_ALTITUDE;
    let orbital_velocity = (G * M_EARTH / orbital_radius).sqrt();
    
    let initial_state = State::new(
        SATELLITE_MASS,
        na::Vector3::new(orbital_radius, 0.0, 0.0),
        na::Vector3::new(0.0, orbital_velocity, 0.0),
    );
    
    let dt = 0.10; // Time step (s)
    let simulation_time = 90.0 * 60.0; // 90 minutes (roughly one orbit)
    let steps = (simulation_time / dt) as usize;
    
    let mut state = initial_state;
    let initial_energy = calculate_energy(&state);
    let initial_angular_momentum = calculate_angular_momentum(&state);
    
    for i in 0..steps {
        if i % 600 == 0 {
            let current_energy = calculate_energy(&state);
            let current_angular_momentum = calculate_angular_momentum(&state);
            
            let energy_error = (current_energy - initial_energy).abs() / initial_energy.abs();
            let angular_momentum_error = (current_angular_momentum - initial_angular_momentum).magnitude() 
                / initial_angular_momentum.magnitude();
            
            println!(
                "Time: {:.0}s\n\
                Position (km): ({:.2}, {:.2}, {:.2})\n\
                Energy relative error: {:.2e}\n\
                Angular momentum relative error: {:.2e}\n", 
                i as f64 * dt,
                state.position[0] / 1000.0,
                state.position[1] / 1000.0,
                state.position[2] / 1000.0,
                energy_error,
                angular_momentum_error
            );
        }
        state = integrators::rk4::integrate(&state, dt);
    }
} 