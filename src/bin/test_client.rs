use bytes::BytesMut;
use fordragon_backend::net::protocol::encode::{BBEncodable, ByteEncoder};
use fordragon_backend::net::protocol::opcode::NetworkRecvOpCode;
use fordragon_backend::user::session::DefaultSessionManager;
use sha2::Digest;
use std::convert::TryFrom;
use std::io::{stdin, BufRead, BufReader};
use std::net::SocketAddr;
use std::str::FromStr;
use tokio::fs;
use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;

const LOCALHOST: &str = "127.0.0.1";
const PORT: &str = "47331";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", LOCALHOST, PORT).parse::<SocketAddr>()?;
    let mut tcp_stream = TcpStream::connect(&addr).await?;
    let (mut socket, mut stream) = tcp_stream.into_split();

    println!(
        r#"Commands are:
    ptc: Protocol Type
    str: String encoding
    u16: 2 bytes number
    u32: 4 bytes number

    send: Send the buffer
    clear: Clear the buffer"#
    );
    let mut buf = BytesMut::new();
    let mut encoder = ByteEncoder::new(&mut buf);
    let mut vec_byte: Vec<u8> = Vec::new();
    let mut stored_lines = Vec::new();

    tokio::spawn(async move {
        println!("Spawned reader");
        loop {
            let mut buf = [0 as u8;1400];
            match socket.read(buf.as_mut()).await {
                Ok(n) if n == 0 => (),
                Ok(n) => {
                    let mut zero_counter = 0;
                    let mut truncate_index = 0;
                    for a in buf.iter() {
                       if *a == 0 {
                           zero_counter += 1;
                       }
                        if zero_counter == 5 {
                           break;
                        }
                        truncate_index += 1;
                    }
                    println!("Message received: {:?}", BytesMut::from(&buf[0..truncate_index]));
                }
                Err(e) => {
                    eprintln!("Error while reading socket");
                }
            }
        }
    });

    for line in stdin().lock().lines() {
        let line = line?;
        if line.len() < 3 {
            panic!("Invalid prefix");
        }
        if line.contains("send") {
            println!("Message sent {:?}", vec_byte.as_slice());

            stream.write_all(encoder.buf()).await?;
            stream.flush().await?;
            continue;
        } else if line.contains("clear") {
            println!("Buffer cleared");
            buf = BytesMut::new();
            encoder = ByteEncoder::new(&mut buf);
            continue;
        }

        match &line.as_str()[0..3] {
            "ptc" => {
                let op = u16::from_str(&line.as_str()[3..]).expect("Invalid op code number");
                let op_code = NetworkRecvOpCode::try_from(op).expect("Invalid op code");
                encoder.encode(&op_code);
            }

            "str" => encoder.encode_str(&line.as_str()[3..]),
            "u16" =>
                encoder.encode_u16(u16::from_str(&line.as_str()[3..]).unwrap()),
            "u32" =>
                encoder.encode_u32(u32::from_str(&line.as_str()[3..]).unwrap()),
            _ => return panic!("Invalid prefix, use str/u16/u32"),
        }
        let line_without_prefix = &line.as_str()[3..];
        stored_lines.push(line_without_prefix.to_string());
        println!("Stored lines {:?}", stored_lines);
    }

    Ok(())
}
