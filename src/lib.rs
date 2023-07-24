mod bytes;
mod gym;
mod rocketsim;
mod socket;

use crate::{
    bytes::{BoostPad, BoostPadState},
    gym::GymState,
};
use bytes::{BallState, CarConfig, CarInfo, CarState, GameState, Team, Vec3, WheelPairConfig};
use glam::{Mat3A, Quat};
use gym::BOOST_PADS_LENGTH;
use pyo3::prelude::*;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    RwLock,
};

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

const TICK_RATE: f32 = 1. / 120.;

static TICK_COUNT: AtomicU64 = AtomicU64::new(0);
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

#[inline]
fn array_to_quat(array: [f32; 4]) -> Quat {
    Quat::from_xyzw(-array[1], -array[2], array[3], array[0])
}

/// Reads the RLGym state and sends it to RLViser to render
#[pyfunction]
fn render_rlgym(gym_state: GymState) {
    // construct the game state
    let game_state = GameState {
        tick_count: TICK_COUNT.fetch_add(1, Ordering::SeqCst),
        tick_rate: TICK_RATE,
        ball: BallState {
            pos: gym_state.ball.position.into(),
            vel: gym_state.ball.linear_velocity.into(),
            ang_vel: gym_state.ball.angular_velocity.into(),
        },
        ball_rot: array_to_quat(gym_state.ball.quaternion),
        pads: BOOST_PAD_LOCATIONS
            .read()
            .unwrap()
            .into_iter()
            .zip(gym_state.boost_pads)
            .map(|(position, is_active)| BoostPad {
                position,
                is_big: position.z == 73.,
                state: BoostPadState {
                    is_active: is_active > 0.5,
                    ..Default::default()
                },
            })
            .collect(),
        cars: gym_state
            .players
            .into_iter()
            .enumerate()
            .map(|(id, player)| CarInfo {
                id: id as u32 + 1,
                team: if player.team_num < 0.5 { Team::Blue } else { Team::Orange },
                state: CarState {
                    pos: player.car_data.position.into(),
                    vel: player.car_data.linear_velocity.into(),
                    ang_vel: player.car_data.angular_velocity.into(),
                    rot_mat: Mat3A::from_quat(array_to_quat(player.car_data.quaternion)).into(),
                    is_on_ground: player.on_ground != 0,
                    is_demoed: player.is_demoed != 0,
                    has_flipped: player.has_flip == 0,
                    has_jumped: player.has_jump == 0,
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
    cars: Vec<(u32, u8, CarConfig, rocketsim::CarState)>,
) {
    let game_state = GameState {
        tick_count,
        tick_rate,
        ball,
        // no way of getting rotation right now
        ball_rot: Quat::IDENTITY,
        pads: BOOST_PAD_LOCATIONS
            .read()
            .unwrap()
            .into_iter()
            .zip(boost_pad_states)
            .map(|(position, is_active)| BoostPad {
                position,
                is_big: position.z == 73.,
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
                state: state.into(),
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
