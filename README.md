## rlviser-py

[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)

Python implementation that manages a UDP connection to RLViser, it launches the [RLViser binary](https://github.com/VirxEC/rlviser) from the current working directory upon first calling any render function.   

Currently able to visualize RLGym GameState objects, but requires the boost pad locations to be sent first.

The backbone of RLGym's `env.render()` functionality.
