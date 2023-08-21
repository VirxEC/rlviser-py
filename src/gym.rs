use std::collections::HashMap;

use pyo3::prelude::*;

pub const BOOST_PADS_LENGTH: usize = 34;

#[derive(FromPyObject, Debug)]
pub struct PhysicsObject {
    pub position: [f32; 3],
    pub linear_velocity: [f32; 3],
    pub angular_velocity: [f32; 3],
    pub _rotation_mtx: Option<[[f32; 3]; 3]>,
}

#[derive(FromPyObject, Debug)]
pub struct Car {
    pub team_num: u8,
    pub boost_amount: f32,
    pub demo_respawn_timer: f32,
    pub physics: PhysicsObject,
}

#[derive(FromPyObject, Debug)]
pub struct GymState {
    pub ball: PhysicsObject,
    pub boost_pad_timers: [f32; BOOST_PADS_LENGTH],
    pub cars: HashMap<String, Car>,
}
