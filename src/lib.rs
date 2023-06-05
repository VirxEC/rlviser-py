mod bytes;
mod gym;
mod socket;

use crate::{
    bytes::{BoostPad, BoostPadState},
    gym::GymState,
};
use bytes::{BallState, CarConfig, CarInfo, CarState, GameState, Team, WheelPairConfig};
use glam::{Mat3A, Quat, Vec3A, EulerRot};
use gym::BOOST_PADS_LENGTH;
use pyo3::prelude::*;
use std::{sync::{
    atomic::{AtomicU64, Ordering},
    RwLock,
}, f32::consts::PI};

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
        quit
    ],
    classes: []
}

const TICK_RATE: f32 = 1. / 120.;

static TICK_COUNT: AtomicU64 = AtomicU64::new(0);
static BOOST_PAD_LOCATIONS: RwLock<[Vec3A; BOOST_PADS_LENGTH]> = RwLock::new([Vec3A::ZERO; BOOST_PADS_LENGTH]);

/// Set the boost pad locations to send to RLViser in each packet
#[pyfunction]
fn set_boost_pad_locations(locations: [[f32; 3]; BOOST_PADS_LENGTH]) {
    BOOST_PAD_LOCATIONS
        .write()
        .unwrap()
        .iter_mut()
        .zip(locations.into_iter())
        .for_each(|(rloc, pyloc)| *rloc = Vec3A::from_array(pyloc));
}

pub const OCTANE: CarConfig = CarConfig {
    hitbox_size: Vec3A::new(120.507, 86.6994, 38.6591),
    hitbox_pos_offset: Vec3A::new(13.87566, 0., 20.755),
    front_wheels: WheelPairConfig {
        wheel_radius: 12.5,
        suspension_rest_length: 38.755,
        connection_point_offset: Vec3A::new(51.25, 25.9, 20.755),
    },
    back_wheels: WheelPairConfig {
        wheel_radius: 15.,
        suspension_rest_length: 37.055,
        connection_point_offset: Vec3A::new(-33.75, 29.5, 20.755),
    },
    dodge_deadzone: 0.5,
};

/// Reads the RLGym state and sends it to RLViser to render
#[pyfunction]
fn render_rlgym(gym_state: GymState) -> PyResult<()> {
    // construct the game state
    let game_state = GameState {
        tick_count: TICK_COUNT.fetch_add(1, Ordering::SeqCst),
        tick_rate: TICK_RATE,
        ball: BallState {
            pos: gym_state.ball.position.into(),
            vel: gym_state.ball.linear_velocity.into(),
            ang_vel: gym_state.ball.angular_velocity.into(),
        },
        ball_rot: Quat::from_array(gym_state.ball.quaternion),
        pads: BOOST_PAD_LOCATIONS
            .read()
            .unwrap()
            .into_iter()
            .zip(gym_state.boost_pads.into_iter())
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
                team: if player.team_num < 0.5 {
                    Team::Blue
                } else {
                    Team::Orange
                },
                state: CarState {
                    pos: player.car_data.position.into(),
                    vel: player.car_data.linear_velocity.into(),
                    ang_vel: player.car_data.angular_velocity.into(),
                    rot_mat: {
                        let (yaw, pitch, roll) = Quat::from_array(player.car_data.quaternion).conjugate().to_euler(EulerRot::ZYX);
                        Mat3A::from_euler(EulerRot::XYZ, yaw, pitch, roll + PI)
                    },
                    ..Default::default()
                },
                config: OCTANE,
            })
            .collect(),
    };

    socket::send_game_state(&game_state).unwrap();

    Ok(())
}

/// Send the quit signal to RLViser
#[pyfunction]
fn quit() {
    socket::quit().unwrap();
}
