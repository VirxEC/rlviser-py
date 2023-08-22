mod bytes;
mod gym;
mod socket;

use crate::{
    bytes::{BoostPad, BoostPadState},
    gym::GymState,
};
use bytes::{BallState, CarConfig, CarInfo, CarState, GameState, Team, Vec3, WheelPairConfig};
use glam::Quat;
use gym::BOOST_PADS_LENGTH;
use pyo3::prelude::*;
use std::{collections::HashMap, sync::RwLock};

macro_rules! pynamedmodule {
    (doc: $doc:literal, name: $name:tt, funcs: [$($func_name:path),*], classes: [$($class_name:ident),*]) => {
        #[doc = $doc]
        #[pymodule]
        #[allow(redundant_semicolons)]
        fn $name(_py: Python, m: &PyModule) -> PyResult<()> {
            $(m.add_function(wrap_pyfunction!($func_name, m)?)?);*;
            $(m.add_class::<$class_name>()?);*;
            Ok(())
        }
    };
}

pynamedmodule! {
    doc: "rlviser_py is a module for interacting with RLViser from Python",
    name: rlviser_py,
    funcs: [
        set_boost_pad_locations,
        render_rlgym,
        render,
        quit
    ],
    classes: []
}

static BOOST_PAD_LOCATIONS: RwLock<[Vec3; BOOST_PADS_LENGTH]> = RwLock::new([Vec3::ZERO; BOOST_PADS_LENGTH]);

/// Set the boost pad locations to send to RLViser in each packet
#[pyfunction]
fn set_boost_pad_locations(locations: [[f32; 3]; BOOST_PADS_LENGTH]) {
    BOOST_PAD_LOCATIONS
        .write()
        .unwrap()
        .iter_mut()
        .zip(locations)
        .for_each(|(rloc, pyloc)| *rloc = Vec3::from_array(pyloc));
}

pub const OCTANE: CarConfig = CarConfig {
    hitbox_size: Vec3::new(120.507, 86.6994, 38.6591),
    hitbox_pos_offset: Vec3::new(13.87566, 0., 20.755),
    front_wheels: WheelPairConfig {
        wheel_radius: 12.5,
        suspension_rest_length: 38.755,
        connection_point_offset: Vec3::new(51.25, 25.9, 20.755),
    },
    back_wheels: WheelPairConfig {
        wheel_radius: 15.,
        suspension_rest_length: 37.055,
        connection_point_offset: Vec3::new(-33.75, 29.5, 20.755),
    },
    dodge_deadzone: 0.5,
};

fn get_sorted_ids<T>(cars: &HashMap<String, T>) -> Vec<&String> {
    let mut ids = cars.keys().collect::<Vec<_>>();
    ids.sort();
    ids
}

/// Reads the RLGym state and sends it to RLViser to render
#[pyfunction]
fn render_rlgym(tick_count: u64, tick_rate: f32, gym_state: GymState) {
    // construct the game state
    let game_state = GameState {
        tick_count,
        tick_rate,
        ball: BallState {
            pos: gym_state.ball.position.into(),
            vel: gym_state.ball.linear_velocity.into(),
            ang_vel: gym_state.ball.angular_velocity.into(),
        },
        ball_rot: Quat::IDENTITY,
        pads: BOOST_PAD_LOCATIONS
            .read()
            .unwrap()
            .into_iter()
            .zip(gym_state.boost_pad_timers)
            .map(|(position, time)| BoostPad {
                position,
                is_big: (position.z - 73.).abs() < f32::EPSILON,
                state: BoostPadState {
                    is_active: time == 0.,
                    ..Default::default()
                },
            })
            .collect(),
        cars: get_sorted_ids(&gym_state.cars)
            .into_iter()
            .map(|id| (id, gym_state.cars.get(id).unwrap()))
            .map(|(id, car)| CarInfo {
                id: id.split('-').last().unwrap().parse::<u32>().unwrap() + 1,
                team: Team::from_u8(car.team_num),
                state: CarState {
                    pos: car.physics.position.into(),
                    vel: car.physics.linear_velocity.into(),
                    ang_vel: car.physics.angular_velocity.into(),
                    rot_mat: car.physics._rotation_mtx.unwrap().into(),
                    is_demoed: car.demo_respawn_timer == 0.,
                    boost: car.boost_amount * 100.,
                    ..Default::default()
                },
                config: OCTANE,
            })
            .collect(),
    };

    socket::send_game_state(&game_state).unwrap();
}

#[pyfunction]
fn render(
    tick_count: u64,
    tick_rate: f32,
    boost_pad_states: [bool; BOOST_PADS_LENGTH],
    ball: BallState,
    ball_rot: [f32; 4],
    cars: Vec<(u32, u8, CarConfig, CarState)>,
) {
    let game_state = GameState {
        tick_count,
        tick_rate,
        ball,
        ball_rot: Quat::from_array(ball_rot),
        pads: BOOST_PAD_LOCATIONS
            .read()
            .unwrap()
            .into_iter()
            .zip(boost_pad_states)
            .map(|(position, is_active)| BoostPad {
                position,
                is_big: (position.z - 73.).abs() < f32::EPSILON,
                state: BoostPadState {
                    is_active,
                    ..Default::default()
                },
            })
            .collect(),
        cars: cars
            .into_iter()
            .map(|(id, team, config, state)| CarInfo {
                id,
                team: Team::from_u8(team),
                config,
                state,
            })
            .collect(),
    };
    socket::send_game_state(&game_state).unwrap();
}

/// Send the quit signal to RLViser
#[pyfunction]
fn quit() {
    socket::quit().unwrap();
}
