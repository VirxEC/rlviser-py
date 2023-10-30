mod bytes;
mod socket;

use crate::bytes::{BoostPad, BoostPadState};
use bytes::{BallState, CarConfig, CarInfo, CarState, GameState, Team, Vec3};
use core::cell::RefCell;
use pyo3::prelude::*;

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

pub const BOOST_PADS_LENGTH: usize = 34;

pynamedmodule! {
    doc: "rlviser_py is a module for interacting with RLViser from Python",
    name: rlviser_py,
    funcs: [
        set_boost_pad_locations,
        render,
        quit
    ],
    classes: []
}

thread_local! {
    static BOOST_PAD_LOCATIONS: RefCell<[Vec3; BOOST_PADS_LENGTH]> = RefCell::new([Vec3::ZERO; BOOST_PADS_LENGTH]);
}

/// Set the boost pad locations to send to RLViser in each packet
#[pyfunction]
fn set_boost_pad_locations(locations: [[f32; 3]; BOOST_PADS_LENGTH]) {
    BOOST_PAD_LOCATIONS.with_borrow_mut(|locs| {
        locs.iter_mut()
            .zip(locations)
            .for_each(|(rloc, pyloc)| *rloc = Vec3::from_array(pyloc));
    });
}

#[pyfunction]
fn render(
    tick_count: u64,
    tick_rate: f32,
    boost_pad_states: [bool; BOOST_PADS_LENGTH],
    ball: BallState,
    cars: Vec<(u32, u8, CarConfig, CarState)>,
) {
    let game_state = GameState {
        tick_count,
        tick_rate,
        ball,
        pads: BOOST_PAD_LOCATIONS.with_borrow(|locs| {
            locs.iter()
                .zip(boost_pad_states)
                .map(|(position, is_active)| BoostPad {
                    position: *position,
                    is_big: (position.z - 73.).abs() < f32::EPSILON,
                    state: BoostPadState {
                        is_active,
                        ..Default::default()
                    },
                })
                .collect()
        }),
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
