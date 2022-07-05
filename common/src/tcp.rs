use std::io;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

pub struct TCPStream {
    pub stream: TcpStream,
}

impl TCPStream {
    pub fn read(&mut self) -> io::Result<String> {
        let mut buffer_size = [0; 4];
        self.stream.read_exact(&mut buffer_size).unwrap();
        let message_size = u32::from_be_bytes(  buffer_size);
        println!("size {}", message_size);
        // let mut buffer_size : [u8; 4] = u32_to_u8_array(message_size);
        //self.stream.read_exact(&mut &message_size)?;
        //let message = String::from_utf8_lossy(&*message_size.to_be_bytes());
        //Ok(message.to_string())
        Ok("".to_string())
    }
    //"hello" -> 7 message len
    pub fn write(&mut self, message: String) -> io::Result<()> {
        let message_len: u32 = message.len() as u32;
        self.stream.write(&message_len.to_be_bytes())?;
        self.stream.write(message.as_bytes())?;
        //self.stream.shutdown(Shutdown::Write)?;
        Ok(())
    }

    // pub fn close(&mut self) -> io::Result<()> {
    //     self.stream.shutdown(Shutdown::Both)?;
    //     Ok(())
    // }
}