use std::{env, os::unix::net::UnixStream, path::PathBuf};

pub struct Display {
    socket: UnixStream,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ConnectError {
    XDG_RUNTIME_DIR_NOT_SET,
    WAYLAND_DISPLAY_NOT_SET,
    SOCKET_CONNECTION_FAILED,
}

impl Display {
    /// Connects to the Wayland server advertised by the `XDG_RUNTIME_DIR` and `WAYLAND_DISPLAY` environment
    /// variables.
    pub fn connect() -> Result<Display, ConnectError> {
        let socket_path = {
            let mut path = PathBuf::from(env::var_os("XDG_RUNTIME_DIR").ok_or(ConnectError::XDG_RUNTIME_DIR_NOT_SET)?);
            path.push(env::var_os("WAYLAND_DISPLAY").ok_or(ConnectError::WAYLAND_DISPLAY_NOT_SET)?);
            path
        };
        println!("Connecting to socket at {:?}", socket_path);

        let socket = UnixStream::connect(socket_path).map_err(|_| ConnectError::SOCKET_CONNECTION_FAILED)?;

        Ok(Display { socket })
    }
}
