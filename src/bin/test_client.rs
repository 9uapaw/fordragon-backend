use tokio::prelude::*;
use tokio::net::TcpStream;
use fordragon_backend::user::session::DefaultSessionManager;
use std::net::SocketAddr;
use sha2::Digest;
use bytes::BytesMut;
use fordragon_backend::net::protocol::opcode::NetworkRecvOpCode;
use fordragon_backend::net::protocol::encode::{BBEncodable, ByteEncoder};
use std::io::{BufReader, stdin, BufRead};
use tokio::io;
use tokio::fs;
use std::str::FromStr;
use std::convert::TryFrom;

const LOCALHOST: &str = "127.0.0.1";
const PORT: &str = "47331";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", LOCALHOST, PORT).parse::<SocketAddr>()?;
    let mut stream = TcpStream::connect(&addr).await?;
    let encoder = ByteEncoder::new();

    println!(r#"Line prefixes are:
    ptc: Protocol Type
    str: String encoding
    u16: 2 bytes number
    u32: 4 bytes number"#);
    let mut vec_byte : Vec<u8> = Vec::new();
    let mut stored_lines = Vec::new();

    for line in  stdin().lock().lines() {
       let line = line?;
        if line.len() < 3 {
            panic!("Invalid prefix");
        }
        if line.contains("send") {
            println!("Message sent");
            println!("{:?}", vec_byte.as_slice());

            stream.write_all(vec_byte.as_slice()).await?;
            stream.flush().await?;
            vec_byte = Vec::new();
        }


        match &line.as_str()[0..3] {
            "ptc" => {
                let op = u16::from_str(&line.as_str()[3..]).expect("Invalid op code number");
                let op_code = NetworkRecvOpCode::try_from(op).expect("Invalid op code");
                vec_byte.extend(op_code.encode_as_bbp())
            }

            "str" => vec_byte.extend(encoder.encode_str(&line.as_str()[3..])),
            "u16" => vec_byte.extend_from_slice(&u16::from_str(&line.as_str()[3..]).unwrap().to_le_bytes()),
            "u32" => vec_byte.extend_from_slice(&u32::from_str(&line.as_str()[3..]).unwrap().to_le_bytes()),
            _ => return panic!("Invalid prefix, use str/u16/u32")
        }
        let line_without_prefix = &line.as_str()[3..];
        stored_lines.push(line_without_prefix.to_string());
        println!("Stored lines {:?}", stored_lines);
    }

    Ok(())
}
