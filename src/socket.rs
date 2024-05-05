use crate::bytes::{FromBytes, GameState, ToBytes};
use std::{
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
    process::Command,
    sync::OnceLock,
};

const RLVISER_PORT: u16 = 45243;
const ROCKETSIM_PORT: u16 = 34254;

#[repr(u8)]
#[derive(Debug)]
pub enum UdpPacketTypes {
    Quit,
    GameState,
    Connection,
    Paused,
    Speed,
    Render,
}

impl UdpPacketTypes {
    const fn new(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(Self::Quit),
            1 => Some(Self::GameState),
            2 => Some(Self::Connection),
            3 => Some(Self::Paused),
            4 => Some(Self::Speed),
            5 => Some(Self::Render),
            _ => None,
        }
    }
}

const RLVISER_PATH: &str = if cfg!(windows) { "./rlviser.exe" } else { "./rlviser" };

static SOCKET: OnceLock<SocketHandler> = OnceLock::new();

#[derive(Default)]
pub struct ReturnMessage {
    pub game_state: Option<GameState>,
    pub speed: Option<f32>,
    pub paused: Option<bool>,
}

impl ReturnMessage {
    pub const NONE: Self = Self {
        game_state: None,
        speed: None,
        paused: None,
    };
}

struct SocketHandler {
    socket: UdpSocket,
    rlviser_addr: SocketAddr,
}

impl SocketHandler {
    pub fn new() -> io::Result<Self> {
        // launch RLViser
        if let Err(e) = Command::new(RLVISER_PATH).spawn() {
            println!("Failed to launch RLViser ({RLVISER_PATH}): {e}");
        }

        let socket = UdpSocket::bind(("0.0.0.0", ROCKETSIM_PORT))?;
        let rlviser_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), RLVISER_PORT);

        socket.set_nonblocking(true)?;
        socket.send_to(&[UdpPacketTypes::Connection as u8], rlviser_addr)?;

        Ok(Self { socket, rlviser_addr })
    }

    fn handle_return_messages(&self) -> io::Result<ReturnMessage> {
        let mut byte_buffer = [0];
        let mut min_game_state_buf = [0; GameState::MIN_NUM_BYTES];
        let mut game_state_buffer = Vec::new();

        let mut game_state = None;
        let mut speed = None;
        let mut paused = None;

        while self.socket.recv_from(&mut byte_buffer).is_ok() {
            let Some(packet_type) = UdpPacketTypes::new(byte_buffer[0]) else {
                println!("Received unknown packet type: {}", byte_buffer[0]);
                break;
            };

            match packet_type {
                UdpPacketTypes::GameState => {
                    self.socket.peek_from(&mut min_game_state_buf)?;

                    let num_bytes = GameState::get_num_bytes(&min_game_state_buf);
                    game_state_buffer.resize(num_bytes, 0);
                    self.socket.recv_from(&mut game_state_buffer)?;

                    game_state = Some(GameState::from_bytes(&game_state_buffer));
                }
                UdpPacketTypes::Speed => {
                    let mut speed_buffer = [0; 4];
                    self.socket.recv_from(&mut speed_buffer)?;

                    speed = Some(f32::from_bytes(&speed_buffer));
                }
                UdpPacketTypes::Paused => {
                    self.socket.recv_from(&mut byte_buffer)?;

                    paused = Some(byte_buffer[0] == 1);
                }
                UdpPacketTypes::Quit | UdpPacketTypes::Render => {
                    panic!("We shouldn't be receiving packets of type {packet_type:?}")
                }
                UdpPacketTypes::Connection => {}
            }
        }

        Ok(ReturnMessage {
            game_state,
            speed,
            paused,
        })
    }

    fn send_game_state(&self, game_state: &GameState) -> io::Result<()> {
        self.socket.send_to(&[UdpPacketTypes::GameState as u8], self.rlviser_addr)?;
        self.socket.send_to(&game_state.to_bytes(), self.rlviser_addr)?;

        Ok(())
    }

    fn report_game_speed(&self, speed: f32) -> io::Result<()> {
        self.socket.send_to(&[UdpPacketTypes::Speed as u8], self.rlviser_addr)?;
        self.socket.send_to(&speed.to_le_bytes(), self.rlviser_addr)?;

        Ok(())
    }

    fn report_game_paused(&self, paused: bool) -> io::Result<()> {
        self.socket.send_to(&[UdpPacketTypes::Paused as u8], self.rlviser_addr)?;
        self.socket.send_to(&[paused as u8], self.rlviser_addr)?;

        Ok(())
    }

    fn send_quit(&self) -> io::Result<()> {
        self.socket.send_to(&[UdpPacketTypes::Quit as u8], self.rlviser_addr)?;

        Ok(())
    }
}

pub fn get_return_messages() -> ReturnMessage {
    let Some(socket_handler) = SOCKET.get() else {
        return ReturnMessage::NONE;
    };

    socket_handler.handle_return_messages().unwrap_or(ReturnMessage::NONE)
}

pub fn send_game_state(game_state: &GameState) -> io::Result<()> {
    let socket_handler = SOCKET.get_or_init(|| SocketHandler::new().unwrap());
    socket_handler.send_game_state(game_state)?;

    Ok(())
}

pub fn report_game_speed(speed: f32) -> io::Result<()> {
    let socket_handler = SOCKET.get_or_init(|| SocketHandler::new().unwrap());
    socket_handler.report_game_speed(speed)?;

    Ok(())
}

pub fn report_game_paused(paused: bool) -> io::Result<()> {
    let socket_handler = SOCKET.get_or_init(|| SocketHandler::new().unwrap());
    socket_handler.report_game_paused(paused)?;

    Ok(())
}

pub fn quit() -> io::Result<()> {
    if let Some(socket_handler) = SOCKET.get() {
        socket_handler.send_quit()?;
    }

    Ok(())
}
