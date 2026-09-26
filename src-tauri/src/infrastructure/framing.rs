// escribir / leer bytes de un stream TCP
use tokio::io::{AsyncBufReadExt, BufReader, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::domain::connection::PeerConnection;

pub async fn write_string_to_stream(s: String, mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let bytes: Vec<u8> = s.into_bytes();

    stream.write_all(&bytes).await?;
    Ok(())
}

pub async fn read_string_from_stream(reader: &mut BufReader<TcpStream>) -> Result<String, Box<dyn std::error::Error>> {
    let mut line = String::new();
    reader.read_line(&mut line).await?;
    Ok(line)
}