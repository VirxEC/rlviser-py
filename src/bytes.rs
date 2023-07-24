use core::panic;

use glam::{Mat3A, Quat, Vec3A};
use pyo3::FromPyObject;

#[derive(Clone, Copy, Debug, Default, FromPyObject)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Self::new(0., 0., 0.);
    pub const X: Self = Self::new(1., 0., 0.);
    pub const Y: Self = Self::new(0., 1., 0.);
    pub const Z: Self = Self::new(0., 0., 1.);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub const fn from_array(array: [f32; 3]) -> Self {
        Self::new(array[0], array[1], array[2])
    }
}

impl From<[f32; 3]> for Vec3 {
    #[inline]
    fn from(value: [f32; 3]) -> Self {
        Self::from_array(value)
    }
}

impl From<Vec3A> for Vec3 {
    #[inline]
    fn from(value: Vec3A) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

#[derive(Clone, Copy, Debug, FromPyObject)]
pub struct RotMat {
    pub forward: Vec3,
    pub right: Vec3,
    pub up: Vec3,
}

impl Default for RotMat {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl RotMat {
    pub const IDENTITY: Self = Self::new(Vec3::X, Vec3::Y, Vec3::Z);

    #[inline]
    pub const fn new(x_axis: Vec3, y_axis: Vec3, z_axis: Vec3) -> Self {
        Self {
            forward: x_axis,
            right: y_axis,
            up: z_axis,
        }
    }
}

impl From<Mat3A> for RotMat {
    #[inline]
    fn from(value: Mat3A) -> Self {
        Self {
            forward: value.x_axis.into(),
            right: value.y_axis.into(),
            up: value.z_axis.into(),
        }
    }
}

#[derive(Clone, Copy, Default, Debug, FromPyObject)]
pub struct BallHitInfo {
    pub is_valid: bool,
    pub relative_pos_on_ball: Vec3,
    pub ball_pos: Vec3,
    pub extra_hit_vel: Vec3,
    pub tick_count_when_hit: u64,
    pub tick_count_when_extra_impulse_applied: u64,
}

#[derive(Clone, Copy, Debug, FromPyObject)]
pub struct BallState {
    pub pos: Vec3,
    pub vel: Vec3,
    pub ang_vel: Vec3,
}

impl Default for BallState {
    #[inline]
    fn default() -> Self {
        Self {
            pos: Vec3::new(0., 0., 92.),
            vel: Vec3::ZERO,
            ang_vel: Vec3::ZERO,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Default, Debug)]
pub enum Team {
    #[default]
    Blue,
    Orange,
}

impl Team {
    #[inline]
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Blue,
            1 => Self::Orange,
            _ => panic!("Invalid team value: {}", value),
        }
    }
}

#[derive(Clone, Copy, Default, Debug, FromPyObject)]
pub struct WheelPairConfig {
    pub wheel_radius: f32,
    pub suspension_rest_length: f32,
    pub connection_point_offset: Vec3,
}

#[derive(Clone, Copy, Default, Debug, FromPyObject)]
pub struct CarConfig {
    pub hitbox_size: Vec3,
    pub hitbox_pos_offset: Vec3,
    pub front_wheels: WheelPairConfig,
    pub back_wheels: WheelPairConfig,
    pub dodge_deadzone: f32,
}

#[derive(Clone, Copy, Debug, Default, FromPyObject)]
pub struct CarControls {
    pub throttle: f32,
    pub steer: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
    pub boost: bool,
    pub jump: bool,
    pub handbrake: bool,
}

#[derive(Clone, Copy, Debug, Default, FromPyObject)]
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

    pub is_flipping: bool,
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
    pub contact_normal: Vec3,
    pub car_contact_id: u32,
    pub car_contact_cooldown_timer: f32,
    pub is_demoed: bool,
    pub demo_respawn_timer: f32,
    pub ball_hit_info: BallHitInfo,
    pub last_controls: CarControls,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct CarInfo {
    pub id: u32,
    pub team: Team,
    pub state: CarState,
    pub config: CarConfig,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct BoostPadState {
    pub is_active: bool,
    pub cooldown: f32,
    pub cur_locked_car_id: u32,
    pub prev_locked_car_id: u32,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct BoostPad {
    pub is_big: bool,
    pub position: Vec3,
    pub state: BoostPadState,
}

#[derive(Default, Debug)]
pub struct GameState {
    pub tick_count: u64,
    pub tick_rate: f32,
    pub ball: BallState,
    pub ball_rot: Quat,
    pub pads: Vec<BoostPad>,
    pub cars: Vec<CarInfo>,
}

trait ToBytesExact<const N: usize>: FromBytesExact {
    fn to_bytes(&self) -> [u8; N];
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

impl ToBytesExact<{ Self::NUM_BYTES }> for Vec3 {
    fn to_bytes(&self) -> [u8; Self::NUM_BYTES] {
        let mut bytes = [0; Self::NUM_BYTES];
        bytes[..4].copy_from_slice(&self.x.to_le_bytes());
        bytes[4..8].copy_from_slice(&self.y.to_le_bytes());
        bytes[8..].copy_from_slice(&self.z.to_le_bytes());
        bytes
    }
}

impl ToBytesExact<{ Self::NUM_BYTES }> for RotMat {
    fn to_bytes(&self) -> [u8; Self::NUM_BYTES] {
        let mut bytes = [0; Self::NUM_BYTES];
        bytes[..Vec3::NUM_BYTES].copy_from_slice(&self.forward.to_bytes());
        bytes[Vec3::NUM_BYTES..Vec3::NUM_BYTES * 2].copy_from_slice(&self.right.to_bytes());
        bytes[Vec3::NUM_BYTES * 2..].copy_from_slice(&self.up.to_bytes());
        bytes
    }
}

impl ToBytesExact<{ Self::NUM_BYTES }> for BallState {
    fn to_bytes(&self) -> [u8; Self::NUM_BYTES] {
        let mut bytes = [0; Self::NUM_BYTES];
        bytes[..Vec3::NUM_BYTES].copy_from_slice(&self.pos.to_bytes());
        bytes[Vec3::NUM_BYTES..Vec3::NUM_BYTES * 2].copy_from_slice(&self.vel.to_bytes());
        bytes[Vec3::NUM_BYTES * 2..].copy_from_slice(&self.ang_vel.to_bytes());
        bytes
    }
}

impl ToBytesExact<{ Self::NUM_BYTES }> for WheelPairConfig {
    fn to_bytes(&self) -> [u8; Self::NUM_BYTES] {
        let mut bytes = [0; Self::NUM_BYTES];
        bytes[..f32::NUM_BYTES].copy_from_slice(&self.wheel_radius.to_le_bytes());
        bytes[f32::NUM_BYTES..f32::NUM_BYTES * 2].copy_from_slice(&self.suspension_rest_length.to_le_bytes());
        bytes[f32::NUM_BYTES * 2..].copy_from_slice(&self.connection_point_offset.to_bytes());
        bytes
    }
}

impl ToBytesExact<{ Self::NUM_BYTES }> for CarConfig {
    fn to_bytes(&self) -> [u8; Self::NUM_BYTES] {
        let mut bytes = [0; Self::NUM_BYTES];
        bytes[..Vec3::NUM_BYTES].copy_from_slice(&self.hitbox_size.to_bytes());
        bytes[Vec3::NUM_BYTES..Vec3::NUM_BYTES * 2].copy_from_slice(&self.hitbox_pos_offset.to_bytes());
        bytes[Vec3::NUM_BYTES * 2..Vec3::NUM_BYTES * 2 + WheelPairConfig::NUM_BYTES]
            .copy_from_slice(&self.front_wheels.to_bytes());
        bytes[Vec3::NUM_BYTES * 2 + WheelPairConfig::NUM_BYTES..Vec3::NUM_BYTES * 2 + WheelPairConfig::NUM_BYTES * 2]
            .copy_from_slice(&self.back_wheels.to_bytes());
        bytes[Vec3::NUM_BYTES * 2 + WheelPairConfig::NUM_BYTES * 2..].copy_from_slice(&self.dodge_deadzone.to_le_bytes());
        bytes
    }
}

impl ToBytesExact<{ Self::NUM_BYTES }> for BallHitInfo {
    fn to_bytes(&self) -> [u8; Self::NUM_BYTES] {
        let mut bytes = [0; Self::NUM_BYTES];
        bytes[..1].copy_from_slice(&(self.is_valid as u8).to_le_bytes());
        bytes[1..1 + Vec3::NUM_BYTES].copy_from_slice(&self.relative_pos_on_ball.to_bytes());
        bytes[1 + Vec3::NUM_BYTES..1 + Vec3::NUM_BYTES * 2].copy_from_slice(&self.ball_pos.to_bytes());
        bytes[1 + Vec3::NUM_BYTES * 2..1 + Vec3::NUM_BYTES * 3].copy_from_slice(&self.extra_hit_vel.to_bytes());
        bytes[1 + Vec3::NUM_BYTES * 3..1 + Vec3::NUM_BYTES * 3 + u64::NUM_BYTES]
            .copy_from_slice(&self.tick_count_when_hit.to_le_bytes());
        bytes[1 + Vec3::NUM_BYTES * 3 + u64::NUM_BYTES..]
            .copy_from_slice(&self.tick_count_when_extra_impulse_applied.to_le_bytes());
        bytes
    }
}

impl ToBytesExact<{ Self::NUM_BYTES }> for CarControls {
    fn to_bytes(&self) -> [u8; Self::NUM_BYTES] {
        let mut bytes = [0; Self::NUM_BYTES];
        bytes[..f32::NUM_BYTES].copy_from_slice(&self.throttle.to_le_bytes());
        bytes[f32::NUM_BYTES..f32::NUM_BYTES * 2].copy_from_slice(&self.steer.to_le_bytes());
        bytes[f32::NUM_BYTES * 2..f32::NUM_BYTES * 3].copy_from_slice(&self.pitch.to_le_bytes());
        bytes[f32::NUM_BYTES * 3..f32::NUM_BYTES * 4].copy_from_slice(&self.yaw.to_le_bytes());
        bytes[f32::NUM_BYTES * 4..f32::NUM_BYTES * 5].copy_from_slice(&self.roll.to_le_bytes());
        bytes[f32::NUM_BYTES * 5..f32::NUM_BYTES * 5 + 1].copy_from_slice(&(self.boost as u8).to_le_bytes());
        bytes[f32::NUM_BYTES * 5 + 1..f32::NUM_BYTES * 5 + 2].copy_from_slice(&(self.jump as u8).to_le_bytes());
        bytes[f32::NUM_BYTES * 5 + 2..].copy_from_slice(&(self.handbrake as u8).to_le_bytes());
        bytes
    }
}

impl ToBytesExact<{ Self::NUM_BYTES }> for CarState {
    fn to_bytes(&self) -> [u8; Self::NUM_BYTES] {
        let mut bytes = [0; Self::NUM_BYTES];
        // pos: Vec3,
        bytes[..Vec3::NUM_BYTES].copy_from_slice(&self.pos.to_bytes());
        // rot_mat: RotMat,
        bytes[Vec3::NUM_BYTES..Vec3::NUM_BYTES + RotMat::NUM_BYTES].copy_from_slice(&self.rot_mat.to_bytes());
        // vel: Vec3,
        bytes[Vec3::NUM_BYTES + RotMat::NUM_BYTES..Vec3::NUM_BYTES * 2 + RotMat::NUM_BYTES]
            .copy_from_slice(&self.vel.to_bytes());
        // ang_vel: Vec3,
        bytes[Vec3::NUM_BYTES * 2 + RotMat::NUM_BYTES..Vec3::NUM_BYTES * 3 + RotMat::NUM_BYTES]
            .copy_from_slice(&self.ang_vel.to_bytes());
        // is_on_ground: bool,
        bytes[Vec3::NUM_BYTES * 3 + RotMat::NUM_BYTES..Vec3::NUM_BYTES * 3 + RotMat::NUM_BYTES + 1]
            .copy_from_slice(&(self.is_on_ground as u8).to_le_bytes());
        // has_jumped: bool,
        bytes[Vec3::NUM_BYTES * 3 + RotMat::NUM_BYTES + 1..Vec3::NUM_BYTES * 3 + RotMat::NUM_BYTES + 2]
            .copy_from_slice(&(self.has_jumped as u8).to_le_bytes());
        // has_double_jumped: bool,
        bytes[Vec3::NUM_BYTES * 3 + RotMat::NUM_BYTES + 2..Vec3::NUM_BYTES * 3 + RotMat::NUM_BYTES + 3]
            .copy_from_slice(&(self.has_double_jumped as u8).to_le_bytes());
        // has_flipped: bool,
        bytes[Vec3::NUM_BYTES * 3 + RotMat::NUM_BYTES + 3..Vec3::NUM_BYTES * 3 + RotMat::NUM_BYTES + 4]
            .copy_from_slice(&(self.has_flipped as u8).to_le_bytes());
        // last_rel_dodge_torque: Vec3,
        bytes[Vec3::NUM_BYTES * 3 + RotMat::NUM_BYTES + 4..Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 4]
            .copy_from_slice(&self.last_rel_dodge_torque.to_bytes());
        // jump_time: f32,
        bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 4..Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 4 + f32::NUM_BYTES]
            .copy_from_slice(&self.jump_time.to_le_bytes());
        // flip_time: f32,
        bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 4 + f32::NUM_BYTES
            ..Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 4 + f32::NUM_BYTES * 2]
            .copy_from_slice(&self.flip_time.to_le_bytes());
        // is_flipping: bool,
        bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 4 + f32::NUM_BYTES * 2
            ..Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 5 + f32::NUM_BYTES * 2]
            .copy_from_slice(&(self.is_flipping as u8).to_le_bytes());
        // is_jumping: bool,
        bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 5 + f32::NUM_BYTES * 2
            ..Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 6 + f32::NUM_BYTES * 2]
            .copy_from_slice(&(self.is_jumping as u8).to_le_bytes());
        // air_time_since_jump: f32,
        bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 6 + f32::NUM_BYTES * 2
            ..Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 6 + f32::NUM_BYTES * 3]
            .copy_from_slice(&self.air_time_since_jump.to_le_bytes());
        // boost: f32,
        bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 6 + f32::NUM_BYTES * 3
            ..Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 6 + f32::NUM_BYTES * 4]
            .copy_from_slice(&self.boost.to_le_bytes());
        // time_spent_boosting: f32,
        bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 6 + f32::NUM_BYTES * 4
            ..Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 6 + f32::NUM_BYTES * 5]
            .copy_from_slice(&self.time_spent_boosting.to_le_bytes());
        // is_supersonic: bool,
        bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 6 + f32::NUM_BYTES * 5
            ..Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 7 + f32::NUM_BYTES * 5]
            .copy_from_slice(&(self.is_supersonic as u8).to_le_bytes());
        // supersonic_time: f32,
        bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 7 + f32::NUM_BYTES * 5
            ..Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 7 + f32::NUM_BYTES * 6]
            .copy_from_slice(&self.supersonic_time.to_le_bytes());
        // handbrake_val: f32,
        bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 7 + f32::NUM_BYTES * 6
            ..Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 7 + f32::NUM_BYTES * 7]
            .copy_from_slice(&self.handbrake_val.to_le_bytes());
        // is_auto_flipping: bool,
        bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 7 + f32::NUM_BYTES * 7
            ..Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 8 + f32::NUM_BYTES * 7]
            .copy_from_slice(&(self.is_auto_flipping as u8).to_le_bytes());
        // auto_flip_timer: f32,
        bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 8 + f32::NUM_BYTES * 7
            ..Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 8 + f32::NUM_BYTES * 8]
            .copy_from_slice(&self.auto_flip_timer.to_le_bytes());
        // auto_flip_torque_scale: f32,
        bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 8 + f32::NUM_BYTES * 8
            ..Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 8 + f32::NUM_BYTES * 9]
            .copy_from_slice(&self.auto_flip_torque_scale.to_le_bytes());
        // has_contact: bool,
        bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 8 + f32::NUM_BYTES * 9
            ..Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 9 + f32::NUM_BYTES * 9]
            .copy_from_slice(&(self.has_world_contact as u8).to_le_bytes());
        // contact_normal: Vec3,
        bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 9 + f32::NUM_BYTES * 9
            ..Vec3::NUM_BYTES * 5 + RotMat::NUM_BYTES + 9 + f32::NUM_BYTES * 9]
            .copy_from_slice(&self.contact_normal.to_bytes());
        // other_car_id: u32,
        bytes[Vec3::NUM_BYTES * 5 + RotMat::NUM_BYTES + 9 + f32::NUM_BYTES * 9
            ..Vec3::NUM_BYTES * 5 + RotMat::NUM_BYTES + 9 + f32::NUM_BYTES * 9 + u32::NUM_BYTES]
            .copy_from_slice(&self.car_contact_id.to_le_bytes());
        // cooldown_timer: f32,
        bytes[Vec3::NUM_BYTES * 5 + RotMat::NUM_BYTES + 9 + f32::NUM_BYTES * 9 + u32::NUM_BYTES
            ..Vec3::NUM_BYTES * 5 + RotMat::NUM_BYTES + 9 + f32::NUM_BYTES * 10 + u32::NUM_BYTES]
            .copy_from_slice(&self.car_contact_cooldown_timer.to_le_bytes());
        // is_demoed: bool,
        bytes[Vec3::NUM_BYTES * 5 + RotMat::NUM_BYTES + 9 + f32::NUM_BYTES * 10 + u32::NUM_BYTES
            ..Vec3::NUM_BYTES * 5 + RotMat::NUM_BYTES + 10 + f32::NUM_BYTES * 10 + u32::NUM_BYTES]
            .copy_from_slice(&(self.is_demoed as u8).to_le_bytes());
        // demo_respawn_timer: f32,
        bytes[Vec3::NUM_BYTES * 5 + RotMat::NUM_BYTES + 10 + f32::NUM_BYTES * 10 + u32::NUM_BYTES
            ..Vec3::NUM_BYTES * 5 + RotMat::NUM_BYTES + 10 + f32::NUM_BYTES * 11 + u32::NUM_BYTES]
            .copy_from_slice(&self.demo_respawn_timer.to_le_bytes());
        // ball_hit_info: BallHitInfo,
        bytes[Vec3::NUM_BYTES * 5 + RotMat::NUM_BYTES + 10 + f32::NUM_BYTES * 11 + u32::NUM_BYTES
            ..Vec3::NUM_BYTES * 5 + RotMat::NUM_BYTES + 10 + f32::NUM_BYTES * 11 + u32::NUM_BYTES + BallHitInfo::NUM_BYTES]
            .copy_from_slice(&self.ball_hit_info.to_bytes());
        // last_controls: CarControls,
        bytes
            [Vec3::NUM_BYTES * 5 + RotMat::NUM_BYTES + 10 + f32::NUM_BYTES * 11 + u32::NUM_BYTES + BallHitInfo::NUM_BYTES..]
            .copy_from_slice(&self.last_controls.to_bytes());
        bytes
    }
}

impl ToBytesExact<{ Self::NUM_BYTES }> for CarInfo {
    fn to_bytes(&self) -> [u8; Self::NUM_BYTES] {
        let mut bytes = [0; Self::NUM_BYTES];
        bytes[..u32::NUM_BYTES].copy_from_slice(&self.id.to_le_bytes());
        bytes[u32::NUM_BYTES..u32::NUM_BYTES + 1].copy_from_slice(&(self.team as u8).to_le_bytes());
        bytes[u32::NUM_BYTES + 1..u32::NUM_BYTES + 1 + CarState::NUM_BYTES].copy_from_slice(&self.state.to_bytes());
        bytes[u32::NUM_BYTES + 1 + CarState::NUM_BYTES..].copy_from_slice(&self.config.to_bytes());
        bytes
    }
}

impl ToBytesExact<{ Self::NUM_BYTES }> for BoostPadState {
    fn to_bytes(&self) -> [u8; Self::NUM_BYTES] {
        let mut bytes = [0; Self::NUM_BYTES];
        bytes[..1].copy_from_slice(&(self.is_active as u8).to_le_bytes());
        bytes[1..5].copy_from_slice(&self.cooldown.to_le_bytes());
        bytes[5..9].copy_from_slice(&self.cur_locked_car_id.to_le_bytes());
        bytes[9..].copy_from_slice(&self.prev_locked_car_id.to_le_bytes());
        bytes
    }
}

impl ToBytesExact<{ Self::NUM_BYTES }> for BoostPad {
    fn to_bytes(&self) -> [u8; Self::NUM_BYTES] {
        let mut bytes = [0; Self::NUM_BYTES];
        bytes[..1].copy_from_slice(&(self.is_big as u8).to_le_bytes());
        bytes[1..1 + Vec3::NUM_BYTES].copy_from_slice(&self.position.to_bytes());
        bytes[1 + Vec3::NUM_BYTES..].copy_from_slice(&self.state.to_bytes());
        bytes
    }
}

impl ToBytes for GameState {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(
            Self::MIN_NUM_BYTES
                + BallState::NUM_BYTES
                + self.pads.len() * BoostPad::NUM_BYTES
                + self.cars.len() * CarInfo::NUM_BYTES,
        );

        bytes.extend(self.tick_count.to_le_bytes());
        bytes.extend(self.tick_rate.to_le_bytes());
        bytes.extend(&(self.pads.len() as u32).to_le_bytes());
        bytes.extend(&(self.cars.len() as u32).to_le_bytes());
        bytes.extend(self.ball.to_bytes());
        bytes.extend(self.ball_rot.x.to_le_bytes());
        bytes.extend(self.ball_rot.y.to_le_bytes());
        bytes.extend(self.ball_rot.z.to_le_bytes());
        bytes.extend(self.ball_rot.w.to_le_bytes());
        bytes.extend(self.pads.iter().flat_map(ToBytesExact::<{ BoostPad::NUM_BYTES }>::to_bytes));
        bytes.extend(self.cars.iter().flat_map(ToBytesExact::<{ CarInfo::NUM_BYTES }>::to_bytes));

        bytes
    }
}

pub trait FromBytesExact {
    const NUM_BYTES: usize;
    fn from_bytes(bytes: &[u8]) -> Self;
}

impl FromBytesExact for f32 {
    const NUM_BYTES: usize = 4;

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Self {
        f32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
}

impl FromBytesExact for u32 {
    const NUM_BYTES: usize = 4;

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Self {
        u32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
}

impl FromBytesExact for u64 {
    const NUM_BYTES: usize = 8;

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Self {
        u64::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])
    }
}

impl FromBytesExact for Vec3 {
    const NUM_BYTES: usize = f32::NUM_BYTES * 3;

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Self {
        Vec3::new(
            f32::from_bytes(&bytes[..f32::NUM_BYTES]),
            f32::from_bytes(&bytes[f32::NUM_BYTES..f32::NUM_BYTES * 2]),
            f32::from_bytes(&bytes[f32::NUM_BYTES * 2..]),
        )
    }
}

impl FromBytesExact for RotMat {
    const NUM_BYTES: usize = Vec3::NUM_BYTES * 3;

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Self {
        RotMat {
            forward: Vec3::from_bytes(&bytes[..Vec3::NUM_BYTES]),
            right: Vec3::from_bytes(&bytes[Vec3::NUM_BYTES..Vec3::NUM_BYTES * 2]),
            up: Vec3::from_bytes(&bytes[Vec3::NUM_BYTES * 2..]),
        }
    }
}

impl FromBytesExact for BallState {
    const NUM_BYTES: usize = Vec3::NUM_BYTES * 3;

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            pos: Vec3::from_bytes(&bytes[..Vec3::NUM_BYTES]),
            vel: Vec3::from_bytes(&bytes[Vec3::NUM_BYTES..Vec3::NUM_BYTES * 2]),
            ang_vel: Vec3::from_bytes(&bytes[Vec3::NUM_BYTES * 2..]),
        }
    }
}

impl FromBytesExact for BoostPadState {
    const NUM_BYTES: usize = 1 + f32::NUM_BYTES + u32::NUM_BYTES * 2;

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            is_active: bytes[0] != 0,
            cooldown: f32::from_bytes(&bytes[1..1 + f32::NUM_BYTES]),
            cur_locked_car_id: u32::from_bytes(&bytes[1 + f32::NUM_BYTES..1 + f32::NUM_BYTES + u32::NUM_BYTES]),
            prev_locked_car_id: u32::from_bytes(&bytes[1 + f32::NUM_BYTES + u32::NUM_BYTES..]),
        }
    }
}

impl FromBytesExact for BoostPad {
    const NUM_BYTES: usize = 1 + Vec3::NUM_BYTES + BoostPadState::NUM_BYTES;

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            is_big: bytes[0] != 0,
            position: Vec3::from_bytes(&bytes[1..1 + Vec3::NUM_BYTES]),
            state: BoostPadState::from_bytes(&bytes[1 + Vec3::NUM_BYTES..Self::NUM_BYTES]),
        }
    }
}

impl FromBytesExact for Team {
    const NUM_BYTES: usize = 1;

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Self {
        match bytes[0] {
            0 => Team::Blue,
            1 => Team::Orange,
            _ => unreachable!(),
        }
    }
}

impl FromBytesExact for BallHitInfo {
    const NUM_BYTES: usize = 1 + Vec3::NUM_BYTES * 3 + u64::NUM_BYTES * 2;

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            is_valid: bytes[0] != 0,
            relative_pos_on_ball: Vec3::from_bytes(&bytes[1..1 + Vec3::NUM_BYTES]),
            ball_pos: Vec3::from_bytes(&bytes[1 + Vec3::NUM_BYTES..1 + Vec3::NUM_BYTES * 2]),
            extra_hit_vel: Vec3::from_bytes(&bytes[1 + Vec3::NUM_BYTES * 2..1 + Vec3::NUM_BYTES * 3]),
            tick_count_when_hit: u64::from_bytes(&bytes[1 + Vec3::NUM_BYTES * 3..1 + Vec3::NUM_BYTES * 3 + u64::NUM_BYTES]),
            tick_count_when_extra_impulse_applied: u64::from_bytes(&bytes[1 + Vec3::NUM_BYTES * 3 + u64::NUM_BYTES..]),
        }
    }
}

impl FromBytesExact for CarControls {
    const NUM_BYTES: usize = f32::NUM_BYTES * 5 + 3;

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            throttle: f32::from_bytes(&bytes[..f32::NUM_BYTES]),
            steer: f32::from_bytes(&bytes[f32::NUM_BYTES..f32::NUM_BYTES * 2]),
            pitch: f32::from_bytes(&bytes[f32::NUM_BYTES * 2..f32::NUM_BYTES * 3]),
            yaw: f32::from_bytes(&bytes[f32::NUM_BYTES * 3..f32::NUM_BYTES * 4]),
            roll: f32::from_bytes(&bytes[f32::NUM_BYTES * 4..f32::NUM_BYTES * 5]),
            boost: bytes[f32::NUM_BYTES * 5] != 0,
            jump: bytes[f32::NUM_BYTES * 5 + 1] != 0,
            handbrake: bytes[f32::NUM_BYTES * 5 + 2] != 0,
        }
    }
}

impl FromBytesExact for CarState {
    const NUM_BYTES: usize = Vec3::NUM_BYTES * 5
        + RotMat::NUM_BYTES
        + 10
        + f32::NUM_BYTES * 11
        + u32::NUM_BYTES
        + BallHitInfo::NUM_BYTES
        + CarControls::NUM_BYTES;

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            pos: Vec3::from_bytes(&bytes[..Vec3::NUM_BYTES]),
            rot_mat: RotMat::from_bytes(&bytes[Vec3::NUM_BYTES..Vec3::NUM_BYTES + RotMat::NUM_BYTES]),
            vel: Vec3::from_bytes(&bytes[Vec3::NUM_BYTES + RotMat::NUM_BYTES..Vec3::NUM_BYTES * 2 + RotMat::NUM_BYTES]),
            ang_vel: Vec3::from_bytes(
                &bytes[Vec3::NUM_BYTES * 2 + RotMat::NUM_BYTES..Vec3::NUM_BYTES * 3 + RotMat::NUM_BYTES],
            ),
            is_on_ground: bytes[Vec3::NUM_BYTES * 3 + RotMat::NUM_BYTES] != 0,
            has_jumped: bytes[Vec3::NUM_BYTES * 3 + RotMat::NUM_BYTES + 1] != 0,
            has_double_jumped: bytes[Vec3::NUM_BYTES * 3 + RotMat::NUM_BYTES + 2] != 0,
            has_flipped: bytes[Vec3::NUM_BYTES * 3 + RotMat::NUM_BYTES + 3] != 0,
            last_rel_dodge_torque: Vec3::from_bytes(
                &bytes[Vec3::NUM_BYTES * 3 + RotMat::NUM_BYTES + 4..Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 4],
            ),
            jump_time: f32::from_bytes(
                &bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 4
                    ..Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 4 + f32::NUM_BYTES],
            ),
            flip_time: f32::from_bytes(
                &bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 4 + f32::NUM_BYTES
                    ..Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 4 + f32::NUM_BYTES * 2],
            ),
            is_flipping: bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 4 + f32::NUM_BYTES * 2] != 0,
            is_jumping: bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 5 + f32::NUM_BYTES * 2] != 0,
            air_time_since_jump: f32::from_bytes(
                &bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 6 + f32::NUM_BYTES * 2
                    ..Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 6 + f32::NUM_BYTES * 3],
            ),
            boost: f32::from_bytes(
                &bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 6 + f32::NUM_BYTES * 3
                    ..Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 6 + f32::NUM_BYTES * 4],
            ),
            time_spent_boosting: f32::from_bytes(
                &bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 6 + f32::NUM_BYTES * 4
                    ..Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 6 + f32::NUM_BYTES * 5],
            ),
            is_supersonic: bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 6 + f32::NUM_BYTES * 5] != 0,
            supersonic_time: f32::from_bytes(
                &bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 7 + f32::NUM_BYTES * 5
                    ..Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 7 + f32::NUM_BYTES * 6],
            ),
            handbrake_val: f32::from_bytes(
                &bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 7 + f32::NUM_BYTES * 5
                    ..Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 7 + f32::NUM_BYTES * 7],
            ),
            is_auto_flipping: bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 7 + f32::NUM_BYTES * 7] != 0,
            auto_flip_timer: f32::from_bytes(
                &bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 8 + f32::NUM_BYTES * 6
                    ..Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 8 + f32::NUM_BYTES * 8],
            ),
            auto_flip_torque_scale: f32::from_bytes(
                &bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 8 + f32::NUM_BYTES * 7
                    ..Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 8 + f32::NUM_BYTES * 9],
            ),
            has_world_contact: bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 8 + f32::NUM_BYTES * 9] != 0,
            contact_normal: Vec3::from_bytes(
                &bytes[Vec3::NUM_BYTES * 4 + RotMat::NUM_BYTES + 9 + f32::NUM_BYTES * 8
                    ..Vec3::NUM_BYTES * 5 + RotMat::NUM_BYTES + 9 + f32::NUM_BYTES * 9],
            ),
            car_contact_id: u32::from_bytes(
                &bytes[Vec3::NUM_BYTES * 5 + RotMat::NUM_BYTES + 9 + f32::NUM_BYTES * 8
                    ..Vec3::NUM_BYTES * 5 + RotMat::NUM_BYTES + 9 + f32::NUM_BYTES * 9 + u32::NUM_BYTES],
            ),
            car_contact_cooldown_timer: f32::from_bytes(
                &bytes[Vec3::NUM_BYTES * 5 + RotMat::NUM_BYTES + 9 + f32::NUM_BYTES * 8 + u32::NUM_BYTES
                    ..Vec3::NUM_BYTES * 5 + RotMat::NUM_BYTES + 9 + f32::NUM_BYTES * 10 + u32::NUM_BYTES],
            ),
            is_demoed: bytes[Vec3::NUM_BYTES * 5 + RotMat::NUM_BYTES + 9 + f32::NUM_BYTES * 10 + u32::NUM_BYTES] != 0,
            demo_respawn_timer: f32::from_bytes(
                &bytes[Vec3::NUM_BYTES * 5 + RotMat::NUM_BYTES + 10 + f32::NUM_BYTES * 9 + u32::NUM_BYTES
                    ..Vec3::NUM_BYTES * 5 + RotMat::NUM_BYTES + 10 + f32::NUM_BYTES * 11 + u32::NUM_BYTES],
            ),
            ball_hit_info: BallHitInfo::from_bytes(
                &bytes[Vec3::NUM_BYTES * 5 + RotMat::NUM_BYTES + 10 + f32::NUM_BYTES * 11 + u32::NUM_BYTES
                    ..Vec3::NUM_BYTES * 5
                        + RotMat::NUM_BYTES
                        + 10
                        + f32::NUM_BYTES * 11
                        + u32::NUM_BYTES
                        + BallHitInfo::NUM_BYTES],
            ),
            last_controls: CarControls::from_bytes(
                &bytes[Vec3::NUM_BYTES * 5
                    + RotMat::NUM_BYTES
                    + 10
                    + f32::NUM_BYTES * 11
                    + u32::NUM_BYTES
                    + BallHitInfo::NUM_BYTES..],
            ),
        }
    }
}

impl FromBytesExact for WheelPairConfig {
    const NUM_BYTES: usize = f32::NUM_BYTES * 2 + Vec3::NUM_BYTES;

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            wheel_radius: f32::from_bytes(&bytes[..f32::NUM_BYTES]),
            suspension_rest_length: f32::from_bytes(&bytes[f32::NUM_BYTES..f32::NUM_BYTES * 2]),
            connection_point_offset: Vec3::from_bytes(&bytes[f32::NUM_BYTES * 2..]),
        }
    }
}

impl FromBytesExact for CarConfig {
    const NUM_BYTES: usize = Vec3::NUM_BYTES * 2 + WheelPairConfig::NUM_BYTES * 2 + f32::NUM_BYTES;

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            hitbox_size: Vec3::from_bytes(&bytes[..Vec3::NUM_BYTES]),
            hitbox_pos_offset: Vec3::from_bytes(&bytes[Vec3::NUM_BYTES..Vec3::NUM_BYTES * 2]),
            front_wheels: WheelPairConfig::from_bytes(
                &bytes[Vec3::NUM_BYTES * 2..Vec3::NUM_BYTES * 2 + WheelPairConfig::NUM_BYTES],
            ),
            back_wheels: WheelPairConfig::from_bytes(
                &bytes
                    [Vec3::NUM_BYTES * 2 + WheelPairConfig::NUM_BYTES..Vec3::NUM_BYTES * 2 + WheelPairConfig::NUM_BYTES * 2],
            ),
            dodge_deadzone: f32::from_bytes(&bytes[Vec3::NUM_BYTES * 2 + WheelPairConfig::NUM_BYTES * 2..]),
        }
    }
}

impl FromBytesExact for CarInfo {
    const NUM_BYTES: usize = u32::NUM_BYTES + Team::NUM_BYTES + CarState::NUM_BYTES + CarConfig::NUM_BYTES;

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            id: u32::from_bytes(&bytes[..u32::NUM_BYTES]),
            team: Team::from_bytes(&bytes[u32::NUM_BYTES..u32::NUM_BYTES + Team::NUM_BYTES]),
            state: CarState::from_bytes(
                &bytes[u32::NUM_BYTES + Team::NUM_BYTES..u32::NUM_BYTES + Team::NUM_BYTES + CarState::NUM_BYTES],
            ),
            config: CarConfig::from_bytes(&bytes[u32::NUM_BYTES + Team::NUM_BYTES + CarState::NUM_BYTES..]),
        }
    }
}

pub trait FromBytes {
    const MIN_NUM_BYTES: usize;
    fn get_num_bytes(bytes: &[u8]) -> usize;
    fn read_tick_count(bytes: &[u8]) -> u64;
    fn read_tick_rate(bytes: &[u8]) -> f32;
    fn read_num_pads(bytes: &[u8]) -> usize;
    fn read_num_cars(bytes: &[u8]) -> usize;
    fn from_bytes(bytes: &[u8]) -> Self;
}

impl FromBytes for GameState {
    const MIN_NUM_BYTES: usize = u64::NUM_BYTES + f32::NUM_BYTES + u32::NUM_BYTES * 2;

    #[inline]
    fn get_num_bytes(bytes: &[u8]) -> usize {
        Self::MIN_NUM_BYTES
            + BallState::NUM_BYTES
            + f32::NUM_BYTES * 4
            + Self::read_num_pads(bytes) * BoostPad::NUM_BYTES
            + Self::read_num_cars(bytes) * CarInfo::NUM_BYTES
    }

    #[inline]
    fn read_tick_count(bytes: &[u8]) -> u64 {
        u64::from_bytes(&bytes[..u64::NUM_BYTES])
    }

    #[inline]
    fn read_tick_rate(bytes: &[u8]) -> f32 {
        f32::from_bytes(&bytes[u64::NUM_BYTES..u64::NUM_BYTES + f32::NUM_BYTES])
    }

    #[inline]
    fn read_num_pads(bytes: &[u8]) -> usize {
        u32::from_bytes(&bytes[u64::NUM_BYTES + f32::NUM_BYTES..u64::NUM_BYTES + f32::NUM_BYTES + u32::NUM_BYTES]) as usize
    }

    #[inline]
    fn read_num_cars(bytes: &[u8]) -> usize {
        u32::from_bytes(&bytes[u64::NUM_BYTES + f32::NUM_BYTES + u32::NUM_BYTES..Self::MIN_NUM_BYTES]) as usize
    }

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            tick_count: Self::read_tick_count(bytes),
            tick_rate: Self::read_tick_rate(bytes),
            ball: BallState::from_bytes(&bytes[Self::MIN_NUM_BYTES..Self::MIN_NUM_BYTES + BallState::NUM_BYTES]),
            ball_rot: Quat::from_xyzw(
                f32::from_bytes(
                    &bytes[Self::MIN_NUM_BYTES + BallState::NUM_BYTES
                        ..Self::MIN_NUM_BYTES + BallState::NUM_BYTES + f32::NUM_BYTES],
                ),
                f32::from_bytes(
                    &bytes[Self::MIN_NUM_BYTES + BallState::NUM_BYTES + f32::NUM_BYTES
                        ..Self::MIN_NUM_BYTES + BallState::NUM_BYTES + f32::NUM_BYTES * 2],
                ),
                f32::from_bytes(
                    &bytes[Self::MIN_NUM_BYTES + BallState::NUM_BYTES + f32::NUM_BYTES * 2
                        ..Self::MIN_NUM_BYTES + BallState::NUM_BYTES + f32::NUM_BYTES * 3],
                ),
                f32::from_bytes(
                    &bytes[Self::MIN_NUM_BYTES + BallState::NUM_BYTES + f32::NUM_BYTES * 3
                        ..Self::MIN_NUM_BYTES + BallState::NUM_BYTES + f32::NUM_BYTES * 4],
                ),
            ),
            pads: bytes[Self::MIN_NUM_BYTES + BallState::NUM_BYTES + f32::NUM_BYTES * 4
                ..Self::MIN_NUM_BYTES
                    + BallState::NUM_BYTES
                    + f32::NUM_BYTES * 4
                    + Self::read_num_pads(bytes) * BoostPad::NUM_BYTES]
                .chunks_exact(BoostPad::NUM_BYTES)
                .map(BoostPad::from_bytes)
                .collect(),
            cars: bytes[Self::MIN_NUM_BYTES
                + BallState::NUM_BYTES
                + f32::NUM_BYTES * 4
                + Self::read_num_pads(bytes) * BoostPad::NUM_BYTES..]
                .chunks_exact(CarInfo::NUM_BYTES)
                .map(CarInfo::from_bytes)
                .collect(),
        }
    }
}
