from typing import List, Tuple

def set_boost_pad_locations(locations: List[Tuple[float, float, float]]) -> ...:
    pass

from RocketSim import BallState, CarConfig, CarState, Team

def render(tick_count: int, tick_rate: float, boost_pad_states: List[bool], ball: BallState, cars: List[Tuple[int, Team, CarConfig, CarState]]) -> ...:
    pass

def quit() -> ...:
    pass
