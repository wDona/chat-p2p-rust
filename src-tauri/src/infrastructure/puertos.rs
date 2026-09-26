use std::io::ErrorKind;
use tokio::net::TcpListener;

pub async fn bind_libre(inicio: u16) -> std::io::Result<TcpListener> {
    for puerto in inicio..=u16::MAX {
        match TcpListener::bind(("0.0.0.0", puerto)).await {
            Ok(listener) => return Ok(listener),
            Err(e) if e.kind() == ErrorKind::AddrInUse => continue,
            Err(e) => return Err(e),
        }
    }
    Err(std::io::Error::new(ErrorKind::AddrInUse, "no hay puertos libres"))
}