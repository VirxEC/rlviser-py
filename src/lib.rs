#![warn(clippy::all)]

mod bytes;
mod socket;

use bytes::{
    BallState, BoostPad, BoostPadState, CarConfig, CarInfo, CarState, FromBytes, GameMode, GameState, TBall, TCar, Team,
    Vec3,
};
use core::cell::RefCell;
use pyo3::prelude::*;

macro_rules! pynamedmodule {
    (doc: $doc:literal, name: $name:tt, funcs: [$($func_name:path),*], vars: [$(($var_name:literal, $value:expr)),*]) => {
        #[doc = $doc]
        #[pymodule]
        #[allow(redundant_semicolons)]
        fn $name(_py: Python, m: &PyModule) -> PyResult<()> {
            $(m.add_function(wrap_pyfunction!($func_name, m)?)?);*;
            $(m.add($var_name, $value)?);*;
            Ok(())
        }
    };
}

pynamedmodule! {
    doc: "rlviser_py is a module for interacting with RLViser from Python",
    name: rlviser_py,
    funcs: [
        set_boost_pad_locations,
        get_state_set,
        render,
        quit
    ],
    vars: [
        ("__version__", env!("CARGO_PKG_VERSION"))
    ]
}

thread_local! {
    static BOOST_PAD_LOCATIONS: RefCell<Vec<Vec3>> = RefCell::new(Vec::new());
}

/// Set the boost pad locations to send to RLViser in each packet
#[pyfunction]
fn set_boost_pad_locations(locations: Vec<[f32; 3]>) {
    BOOST_PAD_LOCATIONS.with_borrow_mut(|locs| {
        locs.resize(locations.len(), Vec3::ZERO);
        locs.iter_mut()
            .zip(locations)
            .for_each(|(rloc, pyloc)| *rloc = Vec3::from_array(pyloc));
    });
}

#[pyfunction]
fn get_state_set() -> Option<(Vec<f32>, TBall, Vec<TCar>)> {
    let game_state = socket::get_state_set()?;

    let pads = game_state.pads.into_iter().map(|pad| pad.state.cooldown).collect::<Vec<_>>();
    let cars = game_state.cars.into_iter().map(CarInfo::to_array).collect::<Vec<_>>();

    Some((pads, game_state.ball.to_array(), cars))
}

type Car = (u32, u8, CarConfig, CarState);

#[pyfunction]
fn render(tick_count: u64, tick_rate: f32, game_mode: u8, boost_pad_states: Vec<bool>, ball: BallState, cars: Vec<Car>) {
    let game_state = GameState {
        tick_count,
        tick_rate,
        game_mode: GameMode::from_bytes(&[game_mode]),
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
