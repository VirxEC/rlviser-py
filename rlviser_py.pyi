from typing import List, Tuple

from RocketSim import BallState, CarConfig, CarState, GameMode, Team

def set_boost_pad_locations(locations: List[Tuple[float, float, float]]) -> ...:
    pass

def render(tick_count: int, tick_rate: float, game_mode: GameMode, boost_pad_states: List[bool], ball: BallState, cars: List[Tuple[int, Team, CarConfig, CarState]]) -> ...:
    pass

def quit() -> ...:
    pass
