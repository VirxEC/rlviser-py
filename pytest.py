import time

import rlviser_py as vis
import RocketSim as rs

# Create example arena
arena = rs.Arena(rs.GameMode.SOCCAR)

# Set boost pad locations
vis.set_boost_pad_locations([pad.get_pos().as_tuple() for pad in arena.get_boost_pads()])

# Setup example arena
car = arena.add_car(rs.Team.BLUE)
car.set_state(rs.CarState(pos=rs.Vec(z=17), vel=rs.Vec(x=50), boost=100))
arena.ball.set_state(rs.BallState(pos=rs.Vec(y=400, z=100), ang_vel=rs.Vec(x=5)))
car.set_controls(rs.CarControls(throttle=1, steer=1, boost=True))

# Run for 3 seconds
TIME = 3

steps = 0
starttime = time.time()
for i in range(round(TIME * arena.tick_rate)):
    arena.step(1)

    # Render the current game state
    pad_states = [pad.get_state().is_active for pad in arena.get_boost_pads()]
    car_data = [(car.id, car.team, car.get_config(), car.get_state()) for car in arena.get_cars()]
    vis.render(steps, arena.tick_rate, pad_states, arena.ball.get_state(), car_data)

    # sleep to simulate running real time (it will run a LOT after otherwise)
    time.sleep(max(0, starttime + steps / arena.tick_rate - time.time()))
    steps += 1

# Tell RLViser to exit
print("Exiting...")
vis.quit()
