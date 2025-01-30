use nalgebra as na;

#[allow(dead_code)] // TODO: Remove this once we have a proper quaternion implementation.
#[derive(Debug, Clone)]
pub struct State {
    pub mass: f64,
    pub position: na::Vector3<f64>,
    pub velocity: na::Vector3<f64>,
    pub quaternion: na::UnitQuaternion<f64>,
    pub angular_velocity: na::Vector3<f64>,
}

impl State {
    pub fn new(mass: f64, position: na::Vector3<f64>, velocity: na::Vector3<f64>) -> Self {
        State {
            mass,
            position,
            velocity,
            quaternion: na::UnitQuaternion::identity(),
            angular_velocity: na::Vector3::zeros(),
        }
    }
} 