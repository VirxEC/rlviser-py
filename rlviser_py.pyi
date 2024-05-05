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
    """
    Sequence[float] - Boost pad states, 0 for full and some positive value for the time in seconds until it respawns
    TBall - Ball state
    Sequence[TCar] - Car states
    """
    pass

def get_game_speed() -> float:
    """
    Returns the current wanted game speed. Default is 1.0 if there has been no request from RLViser to change it.

    The game speed is a multiplier of the game speed, 1.0 is normal speed, 2.0 is double speed, etc.
    """
    pass

def get_game_paused() -> bool:
    """
    Returns the current wanted game pause state. Default is False if there has been no request from RLViser to change it.

    True means the game is paused, False means the game is unpaused.
    """

def report_game_speed(speed: float) -> ...:
    """
    Reports the current game speed to RLViser. This is used to update the game speed in the UI and to properly interpolate the game state.

    NOTE: This is only needed when RLViser did not request the game speed change.
    """
    pass

def report_game_paused(paused: bool) -> ...:
    """
    Reports the current game pause state to RLViser. This is used to update the game pause state in the UI.

    NOTE: This is only needed when RLViser did not request the game pause state change.
    """
    pass

def render(tick_count: int, tick_rate: float, game_mode: GameMode, boost_pad_states: Sequence[bool], ball: BallState, cars: Sequence[Tuple[int, CarState]]) -> ...:
    pass

def quit() -> ...:
    pass
