use crate::bytes::{FromBytes, GameState, ToBytes};
use std::{
    io,
    net::{SocketAddr, UdpSocket},
    process::Command,
    sync::OnceLock,
};

const ROCKETSIM_PORT: u16 = 34254;

#[repr(u8)]
enum UdpPacketTypes {
    Quit,
    GameState,
}

const RLVISER_PATH: &str = if cfg!(windows) { "./rlviser.exe" } else { "./rlviser" };

static SOCKET: OnceLock<(UdpSocket, SocketAddr)> = OnceLock::new();

pub fn init() -> io::Result<(UdpSocket, SocketAddr)> {
    // launch RLViser
    if let Err(e) = Command::new(RLVISER_PATH).spawn() {
        println!("Failed to launch RLViser ({RLVISER_PATH}): {e}");
    }

    // Connect to RLViser
    let socket = UdpSocket::bind(("0.0.0.0", ROCKETSIM_PORT))?;

    println!("Waiting for connection to socket...");
    let mut buf = [0; 1];
    let (_, src) = socket.recv_from(&mut buf)?;

    if buf[0] == 1 {
        println!("Connection established to {src}");
    }

    socket.set_nonblocking(true)?;

    Ok((socket, src))
}

pub fn get_state_set() -> Option<GameState> {
    let (socket, _) = SOCKET.get()?;

    let mut min_state_set_buf = [0; GameState::MIN_NUM_BYTES];
    let mut state_set_buf = Vec::new();

    while let Ok((num_bytes, _)) = socket.peek_from(&mut min_state_set_buf) {
        if num_bytes == 1 {
            // We got a connection and not a game state
            // So clear the byte from the socket buffer and return
            let mut buf = [0];
            socket.recv_from(&mut buf).ok()?;
            continue;
        }

        // the socket sent data back
        // this is the other side telling us to update the game state
        let num_bytes = GameState::get_num_bytes(&min_state_set_buf);
        state_set_buf = vec![0; num_bytes];
        socket.recv_from(&mut state_set_buf).ok()?;
    }

    // the socket didn't send data back
    if state_set_buf.is_empty() {
        return None;
    }

    Some(GameState::from_bytes(&state_set_buf))
}

pub fn send_game_state(game_state: &GameState) -> io::Result<()> {
    let (socket, src) = SOCKET.get_or_init(|| init().unwrap());

    socket.send_to(&[UdpPacketTypes::GameState as u8], src)?;
    socket.send_to(&game_state.to_bytes(), src)?;

    Ok(())
}

pub fn quit() -> io::Result<()> {
    if let Some((socket, src)) = SOCKET.get() {
        socket.send_to(&[UdpPacketTypes::Quit as u8], src)?;
    }

    Ok(())
}
