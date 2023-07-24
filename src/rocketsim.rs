use pyo3::FromPyObject;

use crate::bytes::{BallHitInfo, CarControls, CarState as bCarState, RotMat, Vec3};

#[derive(Clone, Copy, Debug, FromPyObject)]
pub struct CarState {
    pub pos: Vec3,
    pub rot_mat: RotMat,
    pub vel: Vec3,
    pub ang_vel: Vec3,
    pub is_on_ground: bool,
    pub has_jumped: bool,
    pub has_double_jumped: bool,
    pub has_flipped: bool,
    pub last_rel_dodge_torque: Vec3,
    pub jump_time: f32,
    pub flip_time: f32,
    // pub is_flipping: bool,
    pub is_jumping: bool,
    pub air_time_since_jump: f32,
    pub boost: f32,
    pub time_spent_boosting: f32,
    pub is_supersonic: bool,
    pub supersonic_time: f32,
    pub handbrake_val: f32,
    pub is_auto_flipping: bool,
    pub auto_flip_timer: f32,
    pub auto_flip_torque_scale: f32,
    pub has_world_contact: bool,
    // pub contact_normal: Vec,
    pub car_contact_id: u32,
    pub car_contact_cooldown_timer: f32,
    pub is_demoed: bool,
    pub demo_respawn_timer: f32,
    pub ball_hit_info: BallHitInfo,
    pub last_controls: CarControls,
}

impl From<CarState> for bCarState {
    #[inline]
    fn from(value: CarState) -> Self {
        Self {
            pos: value.pos,
            rot_mat: value.rot_mat,
            vel: value.vel,
            ang_vel: value.ang_vel,
            is_on_ground: value.is_on_ground,
            has_jumped: value.has_jumped,
            has_double_jumped: value.has_double_jumped,
            has_flipped: value.has_flipped,
            last_rel_dodge_torque: value.last_rel_dodge_torque,
            jump_time: value.jump_time,
            flip_time: value.flip_time,
            is_jumping: value.is_jumping,
            air_time_since_jump: value.air_time_since_jump,
            boost: value.boost,
            time_spent_boosting: value.time_spent_boosting,
            is_supersonic: value.is_supersonic,
            supersonic_time: value.supersonic_time,
            handbrake_val: value.handbrake_val,
            is_auto_flipping: value.is_auto_flipping,
            auto_flip_timer: value.auto_flip_timer,
            auto_flip_torque_scale: value.auto_flip_torque_scale,
            has_world_contact: value.has_world_contact,
            car_contact_id: value.car_contact_id,
            car_contact_cooldown_timer: value.car_contact_cooldown_timer,
            is_demoed: value.is_demoed,
            demo_respawn_timer: value.demo_respawn_timer,
            ball_hit_info: value.ball_hit_info,
            last_controls: value.last_controls,
            ..Default::default()
        }
    }
}
