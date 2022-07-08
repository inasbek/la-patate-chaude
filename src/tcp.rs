use std::io::Error;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

use crate::message;

pub fn read(stream: &mut TcpStream) -> Result<message::Message, Error> {
    let mut size = [0; 4];
    stream.read_exact(&mut size)?;
    let size = u32::from_be_bytes(size);

    let mut buffer = vec![0; size as usize];
    stream.read_exact(&mut buffer)?;

    let string_message = String::from_utf8(buffer).unwrap();
    let deserialized_msg: message::Message = serde_json::from_str(&string_message)?;
    println!("{:?}", deserialized_msg);
    Ok(deserialized_msg)
}

pub fn write(stream: &mut TcpStream, message: &message::Message) -> Result<(), Error> {
    let message_str = serde_json::to_string(message)?;
    let message_len: u32 = message_str.len() as u32;

    stream.write(&message_len.to_be_bytes())?;
    stream.write(message_str.as_bytes())?;

    Ok(())
}

pub fn close(stream: &mut TcpStream) -> Result<(), Error> {
    stream.shutdown(Shutdown::Both)?;
    Ok(())
}
