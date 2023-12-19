use pyo3::FromPyObject;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default)]
pub enum GameMode {
    Soccar,
    Hoops,
    Heatseeker,
    Snowday,
    #[default]
    TheVoid,
}

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
    pub const fn new(forward: Vec3, right: Vec3, up: Vec3) -> Self {
        Self { forward, right, up }
    }
}

impl From<[[f32; 3]; 3]> for RotMat {
    #[inline]
    fn from(value: [[f32; 3]; 3]) -> Self {
        Self::new(
            Vec3::new(value[0][0], value[1][0], value[2][0]),
            Vec3::new(value[0][1], value[1][1], value[2][1]),
            Vec3::new(value[0][2], value[1][2], value[2][2]),
        )
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
    pub update_counter: u64,
    pub pos: Vec3,
    pub rot_mat: RotMat,
    pub vel: Vec3,
    pub ang_vel: Vec3,
    pub heatseeker_target_dir: f32,
    pub heatseeker_target_speed: f32,
    pub heatseeker_time_since_hit: f32,
}

impl Default for BallState {
    #[inline]
    fn default() -> Self {
        Self {
            update_counter: 0,
            pos: Vec3::new(0., 0., 93.15),
            rot_mat: RotMat::IDENTITY,
            vel: Vec3::ZERO,
            ang_vel: Vec3::ZERO,
            heatseeker_target_dir: 0.,
            heatseeker_target_speed: 2900.,
            heatseeker_time_since_hit: 0.,
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
            _ => panic!("Invalid team value: {value}"),
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
    pub update_counter: u64,
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
    pub world_contact_normal: Vec3,
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
    pub game_mode: GameMode,
    pub ball: BallState,
    pub pads: Vec<BoostPad>,
    pub cars: Vec<CarInfo>,
}

pub trait FromBytes {
    fn from_bytes(bytes: &[u8]) -> Self;
}

pub trait FromBytesExact: FromBytes {
    const NUM_BYTES: usize;
}

struct ByteReader<'a> {
    idx: usize,
    bytes: &'a [u8],
}

impl<'a> ByteReader<'a> {
    #[inline]
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self { idx: 0, bytes }
    }

    pub fn read<I: FromBytesExact>(&mut self) -> I {
        let item = I::from_bytes(&self.bytes[self.idx..self.idx + I::NUM_BYTES]);
        self.idx += I::NUM_BYTES;
        item
    }

    #[inline]
    pub fn debug_assert_num_bytes(&self, num_bytes: usize) {
        debug_assert_eq!(self.idx, num_bytes, "ByteReader::debug_assert_num_bytes() failed");
    }
}

impl FromBytes for bool {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Self {
        bytes[0] != 0
    }
}

impl FromBytesExact for bool {
    const NUM_BYTES: usize = 1;
}

impl FromBytesExact for f32 {
    const NUM_BYTES: usize = 4;
}

impl FromBytes for f32 {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Self {
        Self::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
}

impl FromBytesExact for u32 {
    const NUM_BYTES: usize = 4;
}

impl FromBytes for u32 {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Self {
        Self::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
}

impl FromBytesExact for u64 {
    const NUM_BYTES: usize = 8;
}

impl FromBytes for u64 {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Self {
        Self::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])
    }
}

impl FromBytesExact for Team {
    const NUM_BYTES: usize = 1;
}

impl FromBytes for Team {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Self {
        match bytes[0] {
            0 => Self::Blue,
            1 => Self::Orange,
            _ => unreachable!(),
        }
    }
}

impl FromBytesExact for GameMode {
    const NUM_BYTES: usize = 1;
}

impl FromBytes for GameMode {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Self {
        match bytes[0] {
            0 => Self::Soccar,
            1 => Self::Hoops,
            2 => Self::Heatseeker,
            3 => Self::Snowday,
            4 => Self::TheVoid,
            _ => unreachable!(),
        }
    }
}

impl FromBytesExact for Vec3 {
    const NUM_BYTES: usize = f32::NUM_BYTES * 3;
}

impl FromBytes for Vec3 {
    fn from_bytes(bytes: &[u8]) -> Self {
        let mut reader = ByteReader::new(bytes);
        Self::new(reader.read(), reader.read(), reader.read())
    }
}

macro_rules! impl_from_bytes_exact {
    ($t:ty, $n:expr, $($p:ident),+) => {
        impl FromBytes for $t {
            fn from_bytes(bytes: &[u8]) -> Self {
                let mut reader = ByteReader::new(bytes);
                let item = Self {
                    $($p: reader.read(),)+
                };
                reader.debug_assert_num_bytes(Self::NUM_BYTES);
                item
            }
        }

        impl FromBytesExact for $t {
            const NUM_BYTES: usize = $n;
        }
    };
}

trait ToBytesExact<const N: usize>: FromBytesExact {
    fn to_bytes(&self) -> [u8; N];
}

struct ByteWriter<const N: usize> {
    idx: usize,
    bytes: [u8; N],
}

impl<const N: usize> ByteWriter<N> {
    #[inline]
    pub const fn new() -> Self {
        Self { idx: 0, bytes: [0; N] }
    }

    pub fn write<I: ToBytesExact<M>, const M: usize>(&mut self, item: &I) {
        self.bytes[self.idx..self.idx + M].copy_from_slice(&item.to_bytes());
        self.idx += M;
    }

    #[inline]
    pub fn inner(self) -> [u8; N] {
        debug_assert_eq!(self.idx, N, "ByteWriter::inner() called before all bytes were written");
        self.bytes
    }
}

impl ToBytesExact<{ Self::NUM_BYTES }> for bool {
    fn to_bytes(&self) -> [u8; Self::NUM_BYTES] {
        [u8::from(*self)]
    }
}

impl ToBytesExact<{ Self::NUM_BYTES }> for u32 {
    fn to_bytes(&self) -> [u8; Self::NUM_BYTES] {
        self.to_le_bytes()
    }
}

impl ToBytesExact<{ Self::NUM_BYTES }> for u64 {
    fn to_bytes(&self) -> [u8; Self::NUM_BYTES] {
        self.to_le_bytes()
    }
}

impl ToBytesExact<{ Self::NUM_BYTES }> for f32 {
    fn to_bytes(&self) -> [u8; Self::NUM_BYTES] {
        self.to_le_bytes()
    }
}

impl ToBytesExact<{ Self::NUM_BYTES }> for Team {
    fn to_bytes(&self) -> [u8; Self::NUM_BYTES] {
        [*self as u8]
    }
}

impl ToBytesExact<{ Self::NUM_BYTES }> for GameMode {
    fn to_bytes(&self) -> [u8; Self::NUM_BYTES] {
        [*self as u8]
    }
}

macro_rules! impl_to_bytes_exact {
    ($t:ty, $($p:ident),+) => {
        impl ToBytesExact<{ Self::NUM_BYTES }> for $t {
            fn to_bytes(&self) -> [u8; Self::NUM_BYTES] {
                let mut writer = ByteWriter::<{ Self::NUM_BYTES }>::new();
                $(writer.write(&self.$p);)+
                writer.inner()
            }
        }
    };
}

impl_to_bytes_exact!(Vec3, x, y, z);

macro_rules! impl_bytes_exact {
    ($t:ty, $n:expr, $($p:ident),+) => {
        impl_from_bytes_exact!($t, $n, $($p),+);
        impl_to_bytes_exact!($t, $($p),+);
    };
}

impl_bytes_exact!(RotMat, Vec3::NUM_BYTES * 3, forward, right, up);
impl_bytes_exact!(
    BallState,
    u64::NUM_BYTES + Vec3::NUM_BYTES * 3 + RotMat::NUM_BYTES + f32::NUM_BYTES * 3,
    update_counter,
    pos,
    rot_mat,
    vel,
    ang_vel,
    heatseeker_target_dir,
    heatseeker_target_speed,
    heatseeker_time_since_hit
);
impl_bytes_exact!(
    BoostPadState,
    1 + f32::NUM_BYTES + u32::NUM_BYTES * 2,
    is_active,
    cooldown,
    cur_locked_car_id,
    prev_locked_car_id
);
impl_bytes_exact!(
    BoostPad,
    1 + Vec3::NUM_BYTES + BoostPadState::NUM_BYTES,
    is_big,
    position,
    state
);
impl_bytes_exact!(
    BallHitInfo,
    1 + Vec3::NUM_BYTES * 3 + u64::NUM_BYTES * 2,
    is_valid,
    relative_pos_on_ball,
    ball_pos,
    extra_hit_vel,
    tick_count_when_hit,
    tick_count_when_extra_impulse_applied
);
impl_bytes_exact!(
    CarControls,
    f32::NUM_BYTES * 5 + 3,
    throttle,
    steer,
    pitch,
    yaw,
    roll,
    boost,
    jump,
    handbrake
);
impl_bytes_exact!(
    CarState,
    u64::NUM_BYTES
        + Vec3::NUM_BYTES * 5
        + RotMat::NUM_BYTES
        + 10
        + f32::NUM_BYTES * 11
        + u32::NUM_BYTES
        + BallHitInfo::NUM_BYTES
        + CarControls::NUM_BYTES,
    update_counter,
    pos,
    rot_mat,
    vel,
    ang_vel,
    is_on_ground,
    has_jumped,
    has_double_jumped,
    has_flipped,
    last_rel_dodge_torque,
    jump_time,
    flip_time,
    is_flipping,
    is_jumping,
    air_time_since_jump,
    boost,
    time_spent_boosting,
    is_supersonic,
    supersonic_time,
    handbrake_val,
    is_auto_flipping,
    auto_flip_timer,
    auto_flip_torque_scale,
    has_world_contact,
    world_contact_normal,
    car_contact_id,
    car_contact_cooldown_timer,
    is_demoed,
    demo_respawn_timer,
    ball_hit_info,
    last_controls
);
impl_bytes_exact!(
    WheelPairConfig,
    f32::NUM_BYTES * 2 + Vec3::NUM_BYTES,
    wheel_radius,
    suspension_rest_length,
    connection_point_offset
);
impl_bytes_exact!(
    CarConfig,
    Vec3::NUM_BYTES * 2 + WheelPairConfig::NUM_BYTES * 2 + f32::NUM_BYTES,
    hitbox_size,
    hitbox_pos_offset,
    front_wheels,
    back_wheels,
    dodge_deadzone
);
impl_bytes_exact!(
    CarInfo,
    u32::NUM_BYTES + Team::NUM_BYTES + CarState::NUM_BYTES + CarConfig::NUM_BYTES,
    id,
    team,
    state,
    config
);

impl FromBytes for GameState {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            tick_count: Self::read_tick_count(bytes),
            tick_rate: Self::read_tick_rate(bytes),
            game_mode: Self::read_game_mode(bytes),
            ball: BallState::from_bytes(&bytes[Self::MIN_NUM_BYTES..Self::MIN_NUM_BYTES + BallState::NUM_BYTES]),
            pads: bytes[Self::MIN_NUM_BYTES + BallState::NUM_BYTES
                ..Self::MIN_NUM_BYTES + BallState::NUM_BYTES + Self::read_num_pads(bytes) * BoostPad::NUM_BYTES]
                .chunks_exact(BoostPad::NUM_BYTES)
                .map(BoostPad::from_bytes)
                .collect(),
            cars: bytes[Self::MIN_NUM_BYTES + BallState::NUM_BYTES + Self::read_num_pads(bytes) * BoostPad::NUM_BYTES..]
                .chunks_exact(CarInfo::NUM_BYTES)
                .map(CarInfo::from_bytes)
                .collect(),
        }
    }
}

impl GameState {
    pub const MIN_NUM_BYTES: usize = u64::NUM_BYTES + f32::NUM_BYTES + 1 + u32::NUM_BYTES * 2;

    #[inline]
    pub fn read_tick_count(bytes: &[u8]) -> u64 {
        u64::from_bytes(&bytes[..u64::NUM_BYTES])
    }

    #[inline]
    pub fn read_tick_rate(bytes: &[u8]) -> f32 {
        f32::from_bytes(&bytes[u64::NUM_BYTES..u64::NUM_BYTES + f32::NUM_BYTES])
    }

    #[inline]
    pub fn read_game_mode(bytes: &[u8]) -> GameMode {
        GameMode::from_bytes(&bytes[(u64::NUM_BYTES + f32::NUM_BYTES)..=(u64::NUM_BYTES + f32::NUM_BYTES)])
    }

    #[inline]
    pub fn read_num_pads(bytes: &[u8]) -> usize {
        u32::from_bytes(&bytes[u64::NUM_BYTES + f32::NUM_BYTES + 1..u64::NUM_BYTES + f32::NUM_BYTES + 1 + u32::NUM_BYTES])
            as usize
    }
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

impl ToBytes for GameState {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(
            Self::MIN_NUM_BYTES
                + BallState::NUM_BYTES
                + self.pads.len() * BoostPad::NUM_BYTES
                + self.cars.len() * CarInfo::NUM_BYTES,
        );

        bytes.extend(self.tick_count.to_bytes());
        bytes.extend(self.tick_rate.to_bytes());
        bytes.extend(self.game_mode.to_bytes());
        bytes.extend(&(self.pads.len() as u32).to_bytes());
        bytes.extend(&(self.cars.len() as u32).to_bytes());
        bytes.extend(self.ball.to_bytes());
        bytes.extend(self.pads.iter().flat_map(ToBytesExact::<{ BoostPad::NUM_BYTES }>::to_bytes));
        bytes.extend(self.cars.iter().flat_map(ToBytesExact::<{ CarInfo::NUM_BYTES }>::to_bytes));

        bytes
    }
}
