from typing import Sequence, Optional, Tuple

from RocketSim import BallState, CarState, GameMode

type TVec3 = Tuple[float, float, float]
"""
The items are (X, Y, Z) respectively
"""
type TRotmat = Tuple[TVec3, TVec3, TVec3]
"""
The items are (forward, right, up) respectively
"""
type TBall = Tuple[TVec3, TRotmat, TVec3, TVec3]
"""
The items are (location, rotation, velocity, angular velocity) respectively
"""
type TCar = Tuple[int, TVec3, TRotmat, TVec3, TVec3, float, bool, bool, bool, float]
"""
The items are (car_id, location, rotation, velocity, angular velocity, boost, has jumped, has double jumped, has flipped, demo respawn timer) respectively
"""

def set_boost_pad_locations(locations: Sequence[TVec3]) -> ...:
    pass

def get_state_set() -> Optional[Tuple[Sequence[float], TBall, Sequence[TCar]]]:
    pass

def render(tick_count: int, tick_rate: float, game_mode: GameMode, boost_pad_states: Sequence[bool], ball: BallState, cars: Sequence[Tuple[int, CarState]]) -> ...:
    pass

def quit() -> ...:
    pass
