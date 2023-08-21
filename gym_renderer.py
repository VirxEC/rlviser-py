from typing import Any

import rlviser_py as rlviser

from rlgym.api.engine.renderer import Renderer
from rlgym.rocket_league.common_values import BOOST_LOCATIONS
from rlgym.rocket_league.engine.game_state import GameState


class RLViserRenderer(Renderer[GameState]):

    def __init__(self, tick_rate=120/8):
        rlviser.set_boost_pad_locations(BOOST_LOCATIONS)
        self.tick_rate = tick_rate
        self.packet_id = 0

    def render(self, state: GameState) -> Any:
        self.packet_id += 1
        # print(state)
        rlviser.render_rlgym(self.packet_id, self.tick_rate, state)

    def close(self):
        rlviser.quit()
