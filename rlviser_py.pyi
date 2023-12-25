from typing import List, Optional, Tuple

from RocketSim import BallState, CarConfig, CarState, GameMode, Team

type TVec3 = Tuple[float, float, float]
type TRotmat = Tuple[TVec3, TVec3, TVec3]
type TBall = Tuple[TVec3, TRotmat, TVec3, TVec3]
type TCar = Tuple[int, TVec3, TRotmat, TVec3, TVec3, float, bool, bool, bool, float]

def set_boost_pad_locations(locations: List[TVec3]) -> ...:
    pass

def get_state_set() -> Optional[Tuple[List[float], TBall, List[TCar]]]:
    pass

def render(tick_count: int, tick_rate: float, game_mode: GameMode, boost_pad_states: List[bool], ball: BallState, cars: List[Tuple[int, CarState]]) -> ...:
    pass

def quit() -> ...:
    pass
