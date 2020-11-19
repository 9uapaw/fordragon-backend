use tokio::prelude::*;
use tokio::net::TcpStream;
use fordragon_backend::user::session::DefaultSessionManager;
use std::net::SocketAddr;
use sha2::Digest;
use bytes::BytesMut;
use fordragon_backend::net::protocol::opcode::NetworkRecvOpCode;
use fordragon_backend::net::protocol::encode::{BBEncodable, ByteEncoder};
use std::io::BufReader;
use tokio::io;
use tokio::prelude::{Future, Stream};
use tokio::fs;

const LOCALHOST: &str = "127.0.0.1";
const PORT: &str = "47331";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", LOCALHOST, PORT).parse::<SocketAddr>()?;
    let mut stream = TcpStream::connect(&addr).await?;
    // let mut temp_hash = sha2::Sha256::new();
    // let encoder = ByteEncoder::new();
    // temp_hash.update(&format!("admin:admin:{}", "secret12345").as_bytes());
    // let hash = format!("{:x}", &temp_hash.finalize());
    // let user = "admin".to_string();
    // let mut bytes_vec = Vec::new();
    // let auth_bytes = &(NetworkRecvOpCode::AUTH as u16).to_le_bytes();
    // println!("{:#?}", auth_bytes);
    // bytes_vec.extend(auth_bytes);
    // bytes_vec.extend(encoder.encode_str("admin"));
    // bytes_vec.extend(encoder.encode_str(hash.as_str()));
    //
    // println!("{:#?}", bytes_vec.as_slice());

    let stdin = io::stdin();
    let reader = BufReader::new
    stream.write_all(bytes_vec.as_slice()).await?;
    stream.flush().await?;

    Ok(())
}
