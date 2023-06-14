use crate::bytes::{GameState, ToBytes};
use once_cell::sync::Lazy;
use std::{
    io,
    net::{SocketAddr, UdpSocket},
    process::Command,
    sync::RwLock,
};

#[repr(u8)]
enum UdpPacketTypes {
    Quit,
    GameState,
}

const RLVISER_PATH: &str = if cfg!(windows) { "./rlviser.exe" } else { "./rlviser" };

static SOCKET: RwLock<Lazy<(UdpSocket, SocketAddr)>> = RwLock::new(Lazy::new(|| init().unwrap()));

pub fn init() -> io::Result<(UdpSocket, SocketAddr)> {
    // launch RLViser
    if let Err(e) = Command::new(RLVISER_PATH).spawn() {
        println!("Failed to launch RLViser ({RLVISER_PATH}): {e}");
    }

    // Connect to RLViser
    let socket = UdpSocket::bind("0.0.0.0:34254")?;

    println!("Waiting for connection to socket...");
    let mut buf = [0; 1];
    let (_, src) = socket.recv_from(&mut buf)?;

    if buf[0] == 1 {
        println!("Connection established to {src}");
    }

    socket.set_nonblocking(true)?;

    Ok((socket, src))
}

pub fn send_game_state(game_state: &GameState) -> io::Result<()> {
    let socket_lock = SOCKET.read().unwrap();
    let (socket, src) = &**socket_lock;

    socket.send_to(&[UdpPacketTypes::GameState as u8], src)?;
    socket.send_to(&game_state.to_bytes(), src)?;

    Ok(())
}

pub fn quit() -> io::Result<()> {
    let socket_lock = SOCKET.read().unwrap();
    let (socket, src) = &**socket_lock;

    socket.send_to(&[UdpPacketTypes::Quit as u8], src)?;
    Ok(())
}
