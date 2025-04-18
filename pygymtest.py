import time
from itertools import chain

import numpy as np
import rlviser_py
from rlgym.api import RLGym
from rlgym.rocket_league.action_parsers import LookupTableAction, RepeatAction
from rlgym.rocket_league.done_conditions import (
    AnyCondition,
    GoalCondition,
    NoTouchTimeoutCondition,
    TimeoutCondition,
)
from rlgym.rocket_league.obs_builders import DefaultObs
from rlgym.rocket_league.reward_functions import CombinedReward, GoalReward, TouchReward
from rlgym.rocket_league.sim import RocketSimEngine
from rlgym.rocket_league.state_mutators import (
    FixedTeamSizeMutator,
    KickoffMutator,
    MutatorSequence,
)

from gym_renderer import RLViserRenderer

if __name__ == "__main__":
    game_speed = 1

    env = RLGym(
        state_mutator=MutatorSequence(
            FixedTeamSizeMutator(blue_size=2, orange_size=2), KickoffMutator()
        ),
        obs_builder=DefaultObs(zero_padding=2),
        action_parser=RepeatAction(LookupTableAction()),
        reward_fn=CombinedReward((GoalReward(), 10.0), (TouchReward(), 0.1)),
        termination_cond=GoalCondition(),
        truncation_cond=AnyCondition(
            TimeoutCondition(300.0), NoTouchTimeoutCondition(30.0)
        ),
        transition_engine=RocketSimEngine(),
        renderer=RLViserRenderer(),
    )

    # simulate 2 episodes
    for _ in range(2):
        obs_dict = env.reset()
        steps = 0
        ep_reward = {agent_id: 0.0 for agent_id in env.agents}
        t0 = time.time()
        while True:
            actions = {}
            for agent_id, (type, action_spaces) in env.action_spaces.items():
                actions[agent_id] = np.random.randint(action_spaces, size=(1,))

            obs_dict, reward_dict, terminated_dict, truncated_dict = env.step(
                actions
            )
            env.render()
            time.sleep(15 / 120 / game_speed)
            steps += 1

            for agent_id, reward in reward_dict.items():
                ep_reward[agent_id] += reward

            if any(chain(terminated_dict.values(), truncated_dict.values())):
                break

            game_speed = rlviser_py.get_game_speed()

        ep_time = time.time() - t0
        print(
            f"Steps per second: {steps / ep_time:.0f} | Episode time: {ep_time:.2f} | Episode Reward: {max(ep_reward.values()):.2f}"
        )

    env.close()
