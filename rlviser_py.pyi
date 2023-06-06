def set_boost_pad_locations(locations: list) -> ...:
    pass

try:
    from rlgym_sim.utils.gamestates.game_state import GameState

    def render_rlgym(gym_state: GameState) -> ...:
        pass
except ImportError:
    pass

def quit() -> ...:
    pass
