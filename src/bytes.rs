use pyo3::FromPyObject;

use crate::flat::rocketsim as fb;

// ---------------------------------------------------------------------------
// Traits for converting between our types and flatbuffer types.
// ---------------------------------------------------------------------------

pub trait ToFlat {
    type Flat;
    fn to_flat(&self) -> Self::Flat;
}

pub trait FromFlat<T> {
    fn from_flat(flat: T) -> Self;
}

// ---------------------------------------------------------------------------
// GameMode
// ---------------------------------------------------------------------------

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GameMode {
    Soccar,
    Hoops,
    HeatSeeker,
    Snowday,
    Dropshot,
    #[default]
    TheVoid,
}

impl GameMode {
    pub const fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Soccar,
            1 => Self::Hoops,
            2 => Self::HeatSeeker,
            3 => Self::Snowday,
            4 => Self::Dropshot,
            _ => Self::TheVoid,
        }
    }
}

impl ToFlat for GameMode {
    type Flat = fb::GameMode;

    fn to_flat(&self) -> Self::Flat {
        match self {
            Self::Soccar => fb::GameMode::Soccar,
            Self::Hoops => fb::GameMode::Hoops,
            Self::HeatSeeker => fb::GameMode::Heatseeker,
            Self::Snowday => fb::GameMode::Snowday,
            Self::Dropshot => fb::GameMode::Dropshot,
            Self::TheVoid => fb::GameMode::TheVoid,
        }
    }
}

impl FromFlat<fb::GameMode> for GameMode {
    fn from_flat(value: fb::GameMode) -> Self {
        match value {
            fb::GameMode::Soccar => Self::Soccar,
            fb::GameMode::Hoops => Self::Hoops,
            fb::GameMode::Heatseeker => Self::HeatSeeker,
            fb::GameMode::Snowday => Self::Snowday,
            fb::GameMode::Dropshot => Self::Dropshot,
            fb::GameMode::TheVoid => Self::TheVoid,
        }
    }
}

// ---------------------------------------------------------------------------
// Vec3
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, FromPyObject)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type TVec3 = [f32; 3];

impl Vec3 {
    pub const ZERO: Self = Self::new(0., 0., 0.);
    pub const X: Self = Self::new(1., 0., 0.);
    pub const Y: Self = Self::new(0., 1., 0.);
    pub const Z: Self = Self::new(0., 0., 1.);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub const fn from_array(array: TVec3) -> Self {
        Self::new(array[0], array[1], array[2])
    }

    pub const fn to_array(self) -> TVec3 {
        [self.x, self.y, self.z]
    }
}

impl From<TVec3> for Vec3 {
    #[inline]
    fn from(value: TVec3) -> Self {
        Self::from_array(value)
    }
}

impl ToFlat for Vec3 {
    type Flat = fb::Vec3;

    fn to_flat(&self) -> Self::Flat {
        fb::Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl FromFlat<fb::Vec3> for Vec3 {
    fn from_flat(value: fb::Vec3) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

// ---------------------------------------------------------------------------
// RotMat / Mat3
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, FromPyObject)]
pub struct RotMat {
    pub forward: Vec3,
    pub right: Vec3,
    pub up: Vec3,
}

pub type TRotMat = [TVec3; 3];

impl Default for RotMat {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl RotMat {
    pub const IDENTITY: Self = Self::new(Vec3::X, Vec3::Y, Vec3::Z);

    #[inline]
    pub const fn new(forward: Vec3, right: Vec3, up: Vec3) -> Self {
        Self { forward, right, up }
    }

    pub const fn to_array(self) -> TRotMat {
        [
            self.forward.to_array(),
            self.right.to_array(),
            self.up.to_array(),
        ]
    }
}

impl From<TRotMat> for RotMat {
    #[inline]
    fn from(value: TRotMat) -> Self {
        Self::new(
            Vec3::new(value[0][0], value[1][0], value[2][0]),
            Vec3::new(value[0][1], value[1][1], value[2][1]),
            Vec3::new(value[0][2], value[1][2], value[2][2]),
        )
    }
}

impl ToFlat for RotMat {
    type Flat = fb::Mat3;

    fn to_flat(&self) -> Self::Flat {
        fb::Mat3 {
            forward: self.forward.to_flat(),
            right: self.right.to_flat(),
            up: self.up.to_flat(),
        }
    }
}

impl FromFlat<fb::Mat3> for RotMat {
    fn from_flat(value: fb::Mat3) -> Self {
        Self::new(
            Vec3::from_flat(value.forward),
            Vec3::from_flat(value.right),
            Vec3::from_flat(value.up),
        )
    }
}

// ---------------------------------------------------------------------------
// PhysState helper (flatbuffer struct – used within BallState / CarState)
// ---------------------------------------------------------------------------

fn fb_phys_state(pos: Vec3, rot_mat: RotMat, vel: Vec3, ang_vel: Vec3) -> fb::PhysState {
    fb::PhysState {
        pos: pos.to_flat(),
        rot_mat: rot_mat.to_flat(),
        vel: vel.to_flat(),
        ang_vel: ang_vel.to_flat(),
    }
}

fn from_fb_phys_state(phys: fb::PhysState) -> (Vec3, RotMat, Vec3, Vec3) {
    (
        Vec3::from_flat(phys.pos),
        RotMat::from_flat(phys.rot_mat),
        Vec3::from_flat(phys.vel),
        Vec3::from_flat(phys.ang_vel),
    )
}

// ---------------------------------------------------------------------------
// BallHitInfo
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Default, Debug, FromPyObject)]
pub struct BallHitInfo {
    pub is_valid: bool,
    pub relative_pos_on_ball: Vec3,
    pub ball_pos: Vec3,
    pub extra_hit_vel: Vec3,
    pub tick_count_when_hit: u64,
    pub tick_count_when_extra_impulse_applied: u64,
}

impl ToFlat for BallHitInfo {
    type Flat = Option<Box<fb::BallHitInfo>>;

    fn to_flat(&self) -> Self::Flat {
        if !self.is_valid {
            return None;
        }
        Some(Box::new(fb::BallHitInfo {
            relative_pos_on_ball: self.relative_pos_on_ball.to_flat(),
            ball_pos: self.ball_pos.to_flat(),
            extra_hit_vel: self.extra_hit_vel.to_flat(),
            tick_count_when_hit: self.tick_count_when_hit,
            tick_count_when_extra_impulse_applied: self.tick_count_when_extra_impulse_applied,
        }))
    }
}

impl FromFlat<Option<Box<fb::BallHitInfo>>> for BallHitInfo {
    fn from_flat(value: Option<Box<fb::BallHitInfo>>) -> Self {
        match value {
            Some(info) => Self {
                is_valid: true,
                relative_pos_on_ball: Vec3::from_flat(info.relative_pos_on_ball),
                ball_pos: Vec3::from_flat(info.ball_pos),
                extra_hit_vel: Vec3::from_flat(info.extra_hit_vel),
                tick_count_when_hit: info.tick_count_when_hit,
                tick_count_when_extra_impulse_applied: info.tick_count_when_extra_impulse_applied,
            },
            None => Self::default(),
        }
    }
}

// ---------------------------------------------------------------------------
// BallState
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, FromPyObject)]
pub struct BallState {
    pub pos: Vec3,
    pub rot_mat: RotMat,
    pub vel: Vec3,
    pub ang_vel: Vec3,
    pub heatseeker_target_dir: f32,
    pub heatseeker_target_speed: f32,
    pub heatseeker_time_since_hit: f32,
    #[pyo3(default)]
    pub dropshot_charge_level: i32,
    #[pyo3(default)]
    pub dropshot_accumulated_hit_force: f32,
    #[pyo3(default)]
    pub dropshot_target_dir: f32,
    #[pyo3(default)]
    pub dropshot_has_damaged: bool,
    #[pyo3(default)]
    pub dropshot_last_damage_tick: u64,
}

pub type TBall = (TVec3, TRotMat, TVec3, TVec3);

impl Default for BallState {
    #[inline]
    fn default() -> Self {
        Self {
            pos: Vec3::new(0., 0., 93.15),
            rot_mat: RotMat::IDENTITY,
            vel: Vec3::ZERO,
            ang_vel: Vec3::ZERO,
            heatseeker_target_dir: 0.,
            heatseeker_target_speed: 2900.,
            heatseeker_time_since_hit: 0.,
            dropshot_charge_level: 0,
            dropshot_accumulated_hit_force: 0.0,
            dropshot_target_dir: 0.0,
            dropshot_has_damaged: false,
            dropshot_last_damage_tick: 0,
        }
    }
}

impl BallState {
    #[inline]
    pub const fn to_array(self) -> TBall {
        (
            self.pos.to_array(),
            self.rot_mat.to_array(),
            self.vel.to_array(),
            self.ang_vel.to_array(),
        )
    }
}

impl ToFlat for BallState {
    type Flat = fb::BallState;

    fn to_flat(&self) -> Self::Flat {
        fb::BallState {
            physics: fb_phys_state(self.pos, self.rot_mat, self.vel, self.ang_vel),
            hs_info: fb::HeatseekerInfo {
                y_target_dir: self.heatseeker_target_dir,
                cur_target_speed: self.heatseeker_target_speed,
                time_since_hit: self.heatseeker_time_since_hit,
            },
            ds_info: fb::DropshotInfo {
                charge_level: self.dropshot_charge_level,
                accumulated_hit_force: self.dropshot_accumulated_hit_force,
                y_target_dir: self.dropshot_target_dir,
                has_damaged: self.dropshot_has_damaged,
                last_damage_tick: self.dropshot_last_damage_tick,
            },
        }
    }
}

impl FromFlat<fb::BallState> for BallState {
    fn from_flat(value: fb::BallState) -> Self {
        let (pos, rot_mat, vel, ang_vel) = from_fb_phys_state(value.physics);
        Self {
            pos,
            rot_mat,
            vel,
            ang_vel,
            heatseeker_target_dir: value.hs_info.y_target_dir,
            heatseeker_target_speed: value.hs_info.cur_target_speed,
            heatseeker_time_since_hit: value.hs_info.time_since_hit,
            dropshot_charge_level: value.ds_info.charge_level,
            dropshot_accumulated_hit_force: value.ds_info.accumulated_hit_force,
            dropshot_target_dir: value.ds_info.y_target_dir,
            dropshot_has_damaged: value.ds_info.has_damaged,
            dropshot_last_damage_tick: value.ds_info.last_damage_tick,
        }
    }
}

// ---------------------------------------------------------------------------
// Team
// ---------------------------------------------------------------------------

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
            _ => panic!("Invalid team value: {value}"),
        }
    }
}

impl ToFlat for Team {
    type Flat = fb::Team;

    fn to_flat(&self) -> Self::Flat {
        match self {
            Self::Blue => fb::Team::Blue,
            Self::Orange => fb::Team::Orange,
        }
    }
}

impl FromFlat<fb::Team> for Team {
    fn from_flat(value: fb::Team) -> Self {
        match value {
            fb::Team::Blue => Self::Blue,
            fb::Team::Orange => Self::Orange,
        }
    }
}

// ---------------------------------------------------------------------------
// WheelPairConfig
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Default, Debug, FromPyObject)]
pub struct WheelPairConfig {
    pub wheel_radius: f32,
    pub suspension_rest_length: f32,
    pub connection_point_offset: Vec3,
}

impl ToFlat for WheelPairConfig {
    type Flat = fb::WheelPairConfig;

    fn to_flat(&self) -> Self::Flat {
        fb::WheelPairConfig {
            wheel_radius: self.wheel_radius,
            suspension_rest_length: self.suspension_rest_length,
            connection_point_offset: self.connection_point_offset.to_flat(),
        }
    }
}

impl FromFlat<fb::WheelPairConfig> for WheelPairConfig {
    fn from_flat(value: fb::WheelPairConfig) -> Self {
        Self {
            wheel_radius: value.wheel_radius,
            suspension_rest_length: value.suspension_rest_length,
            connection_point_offset: Vec3::from_flat(value.connection_point_offset),
        }
    }
}

// ---------------------------------------------------------------------------
// CarConfig
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Default, Debug, FromPyObject)]
pub struct CarConfig {
    pub hitbox_size: Vec3,
    pub hitbox_pos_offset: Vec3,
    pub front_wheels: WheelPairConfig,
    pub back_wheels: WheelPairConfig,
    #[pyo3(default)]
    pub three_wheels: bool,
    pub dodge_deadzone: f32,
}

impl ToFlat for CarConfig {
    type Flat = fb::CarConfig;

    fn to_flat(&self) -> Self::Flat {
        fb::CarConfig {
            hitbox_size: self.hitbox_size.to_flat(),
            hitbox_pos_offset: self.hitbox_pos_offset.to_flat(),
            front_wheels: self.front_wheels.to_flat(),
            back_wheels: self.back_wheels.to_flat(),
            three_wheels: self.three_wheels,
            dodge_deadzone: self.dodge_deadzone,
        }
    }
}

impl FromFlat<fb::CarConfig> for CarConfig {
    fn from_flat(value: fb::CarConfig) -> Self {
        Self {
            hitbox_size: Vec3::from_flat(value.hitbox_size),
            hitbox_pos_offset: Vec3::from_flat(value.hitbox_pos_offset),
            front_wheels: WheelPairConfig::from_flat(value.front_wheels),
            back_wheels: WheelPairConfig::from_flat(value.back_wheels),
            three_wheels: value.three_wheels,
            dodge_deadzone: value.dodge_deadzone,
        }
    }
}

// ---------------------------------------------------------------------------
// CarControls
// ---------------------------------------------------------------------------

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

impl ToFlat for CarControls {
    type Flat = fb::CarControls;

    fn to_flat(&self) -> Self::Flat {
        fb::CarControls {
            throttle: self.throttle,
            steer: self.steer,
            pitch: self.pitch,
            yaw: self.yaw,
            roll: self.roll,
            boost: self.boost,
            jump: self.jump,
            handbrake: self.handbrake,
        }
    }
}

impl FromFlat<fb::CarControls> for CarControls {
    fn from_flat(value: fb::CarControls) -> Self {
        Self {
            throttle: value.throttle,
            steer: value.steer,
            pitch: value.pitch,
            yaw: value.yaw,
            roll: value.roll,
            boost: value.boost,
            jump: value.jump,
            handbrake: value.handbrake,
        }
    }
}

// ---------------------------------------------------------------------------
// CarState
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, FromPyObject)]
pub struct CarState {
    pub pos: Vec3,
    pub rot_mat: RotMat,
    pub vel: Vec3,
    pub ang_vel: Vec3,
    pub is_on_ground: bool,
    pub wheels_with_contact: [bool; 4],
    pub has_jumped: bool,
    pub has_double_jumped: bool,
    pub has_flipped: bool,
    pub flip_rel_torque: Vec3,
    pub jump_time: f32,
    pub flip_time: f32,
    pub is_flipping: bool,
    pub is_jumping: bool,
    pub air_time: f32,
    pub air_time_since_jump: f32,
    pub boost: f32,
    pub time_spent_boosting: f32,
    #[pyo3(default)]
    pub is_boosting: bool,
    #[pyo3(default)]
    pub boosting_time: f32,
    pub is_supersonic: bool,
    pub supersonic_time: f32,
    pub handbrake_val: f32,
    pub is_auto_flipping: bool,
    pub auto_flip_timer: f32,
    pub auto_flip_torque_scale: f32,
    pub has_world_contact: bool,
    pub world_contact_normal: Vec3,
    pub car_contact_id: u32,
    pub car_contact_cooldown_timer: f32,
    pub is_demoed: bool,
    pub demo_respawn_timer: f32,
    pub ball_hit_info: BallHitInfo,
    pub last_controls: CarControls,
}

impl ToFlat for CarState {
    type Flat = Box<fb::CarState>;

    fn to_flat(&self) -> Self::Flat {
        Box::new(fb::CarState {
            physics: fb_phys_state(self.pos, self.rot_mat, self.vel, self.ang_vel),
            is_on_ground: self.is_on_ground,
            wheels_with_contact: fb::WheelsWithContact {
                front_left: self.wheels_with_contact[0],
                front_right: self.wheels_with_contact[1],
                rear_left: self.wheels_with_contact[2],
                rear_right: self.wheels_with_contact[3],
            },
            has_jumped: self.has_jumped,
            has_double_jumped: self.has_double_jumped,
            has_flipped: self.has_flipped,
            flip_rel_torque: self.flip_rel_torque.to_flat(),
            jump_time: self.jump_time,
            flip_time: self.flip_time,
            is_flipping: self.is_flipping,
            is_jumping: self.is_jumping,
            air_time: self.air_time,
            air_time_since_jump: self.air_time_since_jump,
            boost: self.boost,
            time_since_boosted: self.time_spent_boosting,
            is_boosting: self.is_boosting,
            boosting_time: self.boosting_time,
            is_supersonic: self.is_supersonic,
            supersonic_time: self.supersonic_time,
            handbrake_val: self.handbrake_val,
            is_auto_flipping: self.is_auto_flipping,
            auto_flip_timer: self.auto_flip_timer,
            auto_flip_torque_scale: self.auto_flip_torque_scale,
            world_contact_normal: if self.has_world_contact {
                Some(self.world_contact_normal.to_flat())
            } else {
                None
            },
            car_contact: if self.car_contact_id != 0 {
                Some(Box::new(fb::CarContact {
                    other_car_id: u64::from(self.car_contact_id),
                    cooldown_timer: self.car_contact_cooldown_timer,
                }))
            } else {
                None
            },
            is_demoed: self.is_demoed,
            demo_respawn_timer: self.demo_respawn_timer,
            ball_hit_info: self.ball_hit_info.to_flat(),
            last_controls: self.last_controls.to_flat(),
        })
    }
}

impl FromFlat<fb::CarState> for CarState {
    fn from_flat(value: fb::CarState) -> Self {
        let (pos, rot_mat, vel, ang_vel) = from_fb_phys_state(value.physics);
        let ball_hit_info = BallHitInfo::from_flat(value.ball_hit_info);
        let (has_world_contact, world_contact_normal) = match value.world_contact_normal {
            Some(normal) => (true, Vec3::from_flat(normal)),
            None => (false, Vec3::ZERO),
        };
        let (car_contact_id, car_contact_cooldown_timer) = match value.car_contact {
            Some(contact) => (contact.other_car_id as u32, contact.cooldown_timer),
            None => (0, 0.0),
        };

        Self {
            pos,
            rot_mat,
            vel,
            ang_vel,
            is_on_ground: value.is_on_ground,
            wheels_with_contact: [
                value.wheels_with_contact.front_left,
                value.wheels_with_contact.front_right,
                value.wheels_with_contact.rear_left,
                value.wheels_with_contact.rear_right,
            ],
            has_jumped: value.has_jumped,
            has_double_jumped: value.has_double_jumped,
            has_flipped: value.has_flipped,
            flip_rel_torque: Vec3::from_flat(value.flip_rel_torque),
            jump_time: value.jump_time,
            flip_time: value.flip_time,
            is_flipping: value.is_flipping,
            is_jumping: value.is_jumping,
            air_time: value.air_time,
            air_time_since_jump: value.air_time_since_jump,
            boost: value.boost,
            time_spent_boosting: value.time_since_boosted,
            is_boosting: value.is_boosting,
            boosting_time: value.boosting_time,
            is_supersonic: value.is_supersonic,
            supersonic_time: value.supersonic_time,
            handbrake_val: value.handbrake_val,
            is_auto_flipping: value.is_auto_flipping,
            auto_flip_timer: value.auto_flip_timer,
            auto_flip_torque_scale: value.auto_flip_torque_scale,
            has_world_contact,
            world_contact_normal,
            car_contact_id,
            car_contact_cooldown_timer,
            is_demoed: value.is_demoed,
            demo_respawn_timer: value.demo_respawn_timer,
            ball_hit_info,
            last_controls: CarControls::from_flat(value.last_controls),
        }
    }
}

// ---------------------------------------------------------------------------
// CarInfo
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Default, Debug)]
pub struct CarInfo {
    pub id: u32,
    pub team: Team,
    pub state: CarState,
    pub config: CarConfig,
}

pub type TCar = (
    u32,
    TVec3,
    TRotMat,
    TVec3,
    TVec3,
    f32,
    bool,
    bool,
    bool,
    f32,
);

impl CarInfo {
    #[inline]
    pub const fn to_array(self) -> TCar {
        (
            self.id,
            self.state.pos.to_array(),
            self.state.rot_mat.to_array(),
            self.state.vel.to_array(),
            self.state.ang_vel.to_array(),
            self.state.boost,
            self.state.has_jumped,
            self.state.has_double_jumped,
            self.state.has_flipped,
            self.state.demo_respawn_timer,
        )
    }
}

impl ToFlat for CarInfo {
    type Flat = fb::CarInfo;

    fn to_flat(&self) -> Self::Flat {
        fb::CarInfo {
            id: u64::from(self.id),
            team: self.team.to_flat(),
            state: self.state.to_flat(),
            config: self.config.to_flat(),
        }
    }
}

impl FromFlat<fb::CarInfo> for CarInfo {
    fn from_flat(value: fb::CarInfo) -> Self {
        Self {
            id: value.id as u32,
            team: Team::from_flat(value.team),
            state: CarState::from_flat(*value.state),
            config: CarConfig::from_flat(value.config),
        }
    }
}

// ---------------------------------------------------------------------------
// BoostPadState
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Default, Debug)]
pub struct BoostPadState {
    pub is_active: bool,
    pub cooldown: f32,
    pub cur_locked_car_id: u32,
    pub prev_locked_car_id: u32,
}

impl ToFlat for BoostPadState {
    type Flat = fb::BoostPadState;

    fn to_flat(&self) -> Self::Flat {
        fb::BoostPadState {
            is_active: self.is_active,
            cooldown: self.cooldown,
            cur_locked_car: u64::from(self.cur_locked_car_id),
            prev_locked_car_id: u64::from(self.prev_locked_car_id),
        }
    }
}

impl FromFlat<fb::BoostPadState> for BoostPadState {
    fn from_flat(value: fb::BoostPadState) -> Self {
        Self {
            is_active: value.is_active,
            cooldown: value.cooldown,
            cur_locked_car_id: value.cur_locked_car as u32,
            prev_locked_car_id: value.prev_locked_car_id as u32,
        }
    }
}

// ---------------------------------------------------------------------------
// BoostPad
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Default, Debug)]
pub struct BoostPad {
    pub is_big: bool,
    pub position: Vec3,
    pub state: BoostPadState,
}

impl ToFlat for BoostPad {
    type Flat = fb::BoostPadInfo;

    fn to_flat(&self) -> Self::Flat {
        fb::BoostPadInfo {
            config: fb::BoostPadConfig {
                pos: self.position.to_flat(),
                is_big: self.is_big,
            },
            state: self.state.to_flat(),
        }
    }
}

impl FromFlat<fb::BoostPadInfo> for BoostPad {
    fn from_flat(value: fb::BoostPadInfo) -> Self {
        Self {
            is_big: value.config.is_big,
            position: Vec3::from_flat(value.config.pos),
            state: BoostPadState::from_flat(value.state),
        }
    }
}

// ---------------------------------------------------------------------------
// GameState
// ---------------------------------------------------------------------------

#[derive(Default, Debug)]
pub struct GameState {
    pub tick_count: u64,
    pub tick_rate: f32,
    pub game_mode: GameMode,
    pub ball: BallState,
    pub pads: Vec<BoostPad>,
    pub cars: Vec<CarInfo>,
}

impl ToFlat for GameState {
    type Flat = fb::GameState;

    fn to_flat(&self) -> Self::Flat {
        fb::GameState {
            tick_rate: self.tick_rate,
            tick_count: self.tick_count,
            game_mode: self.game_mode.to_flat(),
            cars: Some(self.cars.iter().map(ToFlat::to_flat).collect()),
            ball: self.ball.to_flat(),
            pads: Some(self.pads.iter().map(ToFlat::to_flat).collect()),
            tiles: None,
        }
    }
}

impl FromFlat<fb::GameState> for GameState {
    fn from_flat(value: fb::GameState) -> Self {
        Self {
            tick_count: value.tick_count,
            tick_rate: value.tick_rate,
            game_mode: GameMode::from_flat(value.game_mode),
            ball: BallState::from_flat(value.ball),
            pads: value
                .pads
                .into_iter()
                .flatten()
                .map(BoostPad::from_flat)
                .collect(),
            cars: value
                .cars
                .into_iter()
                .flatten()
                .map(CarInfo::from_flat)
                .collect(),
        }
    }
}
