use crate::bytes::{FromFlat, GameState, ToFlat};
use std::{
    io,
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    process::Command,
    sync::OnceLock,
};
use sysinfo::{ProcessRefreshKind, RefreshKind, System};

use crate::flat::rocketsim as fb;

const RLVISER_PORT: u16 = 45243;
const ROCKETSIM_PORT: u16 = 34254;
const PACKET_SIZE_BYTES: usize = 8;

const RLVISER_PATH: &str = if cfg!(windows) {
    "./rlviser.exe"
} else {
    "./rlviser"
};

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

/// Encode a flatbuffer Message into the wire format:
///   [8-byte big-endian payload length][flatbuffer Packet payload]
fn encode_message(message: fb::Message) -> Vec<u8> {
    let mut builder = planus::Builder::with_capacity(1024);
    let packet = fb::Packet { message };
    let payload = builder.finish(packet, None);
    let data_len_bin = u64::try_from(payload.len()).unwrap().to_be_bytes();

    let mut buffer = Vec::with_capacity(PACKET_SIZE_BYTES + payload.len());
    buffer.extend_from_slice(&data_len_bin);
    buffer.extend_from_slice(payload);
    buffer
}

/// Decode a flatbuffer Packet payload (after the 8-byte header) into a Message.
fn decode_payload(payload: &[u8]) -> io::Result<fb::Message> {
    let packet: fb::Packet = <fb::PacketRef<'_> as planus::ReadAsRoot>::read_as_root(payload)
        .and_then(|p| p.try_into())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
    Ok(packet.message)
}

impl SocketHandler {
    pub fn new() -> io::Result<Self> {
        let sys = System::new_with_specifics(
            RefreshKind::nothing().with_processes(ProcessRefreshKind::nothing()),
        );
        let rlviser_procs = sys.processes_by_exact_name("rlviser".as_ref()).count();

        // launch RLViser if it hasn't been already
        if rlviser_procs == 0
            && let Err(e) = Command::new(RLVISER_PATH).spawn()
        {
            eprintln!("Failed to launch RLViser ({RLVISER_PATH}): {e}");
        }

        let socket = UdpSocket::bind((Ipv4Addr::new(0, 0, 0, 0), ROCKETSIM_PORT))?;
        let rlviser_addr = (Ipv4Addr::new(127, 0, 0, 1), RLVISER_PORT).into();

        socket.send_to(
            &encode_message(fb::Message::Connection(Box::default())),
            rlviser_addr,
        )?;
        socket.set_nonblocking(true)?;

        Ok(Self {
            socket,
            rlviser_addr,
        })
    }

    fn handle_return_messages(&self) -> io::Result<ReturnMessage> {
        let mut header = [0u8; PACKET_SIZE_BYTES];
        let mut buffer = Vec::with_capacity(1024);

        let mut game_state = None;
        let mut speed = None;
        let mut paused = None;

        while self.socket.peek_from(&mut header).is_ok() {
            let packet_size = PACKET_SIZE_BYTES + u64::from_be_bytes(header) as usize;
            buffer.resize(packet_size, 0);

            let (_, _src) = self.socket.recv_from(&mut buffer)?;
            let payload = &buffer[PACKET_SIZE_BYTES..];

            let Ok(message) = decode_payload(payload) else {
                continue;
            };

            match message {
                fb::Message::Connection(_) => {
                    eprintln!("Connection established");
                }
                fb::Message::Speed(s) => {
                    speed = Some(s.speed);
                }
                fb::Message::Paused(p) => {
                    paused = Some(p.paused);
                }
                fb::Message::GameState(gs) => {
                    game_state = Some(GameState::from_flat(*gs));
                }
                fb::Message::Quit(_) => {}
                fb::Message::AddRender(_) | fb::Message::RemoveRender(_) => {}
            }
        }

        Ok(ReturnMessage {
            game_state,
            speed,
            paused,
        })
    }

    fn send_game_state(&self, game_state: &GameState) -> io::Result<()> {
        let fb_gs = game_state.to_flat();
        let bytes = encode_message(fb::Message::GameState(Box::new(fb_gs)));
        self.socket.send_to(&bytes, self.rlviser_addr)?;
        Ok(())
    }

    fn report_game_speed(&self, speed: f32) -> io::Result<()> {
        let bytes = encode_message(fb::Message::Speed(Box::new(fb::Speed { speed })));
        self.socket.send_to(&bytes, self.rlviser_addr)?;
        Ok(())
    }

    fn report_game_paused(&self, paused: bool) -> io::Result<()> {
        let bytes = encode_message(fb::Message::Paused(Box::new(fb::Paused { paused })));
        self.socket.send_to(&bytes, self.rlviser_addr)?;
        Ok(())
    }

    fn send_quit(&self) -> io::Result<()> {
        let bytes = encode_message(fb::Message::Quit(Box::default()));
        self.socket.send_to(&bytes, self.rlviser_addr)?;
        Ok(())
    }
}

pub fn get_return_messages() -> ReturnMessage {
    let Some(socket_handler) = SOCKET.get() else {
        return ReturnMessage::NONE;
    };

    socket_handler
        .handle_return_messages()
        .unwrap_or(ReturnMessage::NONE)
}

pub fn send_game_state(game_state: &GameState) -> io::Result<()> {
    let socket_handler = SOCKET.get_or_init(|| SocketHandler::new().unwrap());
    socket_handler.send_game_state(game_state)
}

pub fn report_game_speed(speed: f32) -> io::Result<()> {
    let socket_handler = SOCKET.get_or_init(|| SocketHandler::new().unwrap());
    socket_handler.report_game_speed(speed)
}

pub fn report_game_paused(paused: bool) -> io::Result<()> {
    let socket_handler = SOCKET.get_or_init(|| SocketHandler::new().unwrap());
    socket_handler.report_game_paused(paused)
}

pub fn launch() -> io::Result<()> {
    if let Err(e) = Command::new(RLVISER_PATH).spawn() {
        eprintln!("Failed to launch RLViser ({RLVISER_PATH}): {e}");
    }

    Ok(())
}

pub fn quit() -> io::Result<()> {
    if let Some(socket_handler) = SOCKET.get() {
        socket_handler.send_quit()?;
    }

    Ok(())
}
