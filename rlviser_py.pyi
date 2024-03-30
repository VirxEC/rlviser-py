from typing import Sequence, Optional, Tuple

from RocketSim import BallState, CarConfig, CarState, GameMode, Team

type TVec3 = Tuple[float, float, float]
type TRotmat = Tuple[TVec3, TVec3, TVec3]
type TBall = Tuple[TVec3, TRotmat, TVec3, TVec3]
type TCar = Tuple[int, TVec3, TRotmat, TVec3, TVec3, float, bool, bool, bool, float]

def set_boost_pad_locations(locations: Sequence[TVec3]) -> ...:
    pass

def get_state_set() -> Optional[Tuple[Sequence[float], TBall, Sequence[TCar]]]:
    pass

def render(tick_count: int, tick_rate: float, game_mode: GameMode, boost_pad_states: Sequence[bool], ball: BallState, cars: Sequence[Tuple[int, CarState]]) -> ...:
    pass

def quit() -> ...:
    pass
