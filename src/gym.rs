use pyo3::prelude::*;
use std::ffi::c_uchar;

pub const BOOST_PADS_LENGTH: usize = 34;

#[derive(FromPyObject, Debug)]
pub struct PhysicsObject {
    pub position: [f32; 3],
    pub quaternion: [f32; 4],
    pub linear_velocity: [f32; 3],
    pub angular_velocity: [f32; 3],
}

#[derive(FromPyObject, Debug)]
pub struct PlayerData {
    pub car_id: u32,
    pub team_num: f32,
    pub match_goals: f32,
    pub match_saves: f32,
    pub match_shots: f32,
    pub match_demolishes: f32,
    pub boost_pickups: f32,
    pub is_demoed: c_uchar,
    pub on_ground: c_uchar,
    pub ball_touched: c_uchar,
    pub has_jump: c_uchar,
    pub has_flip: c_uchar,
    pub boost_amount: f32,
    pub car_data: PhysicsObject,
}

#[derive(FromPyObject, Debug)]
pub struct GymState {
    // pub game_type: f32,
    // pub blue_score: f32,
    // pub orange_score: f32,
    // pub last_touch: f32,
    pub ball: PhysicsObject,
    pub boost_pads: [f32; BOOST_PADS_LENGTH],
    pub players: Vec<PlayerData>,
}
