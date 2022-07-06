use std::io;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

pub struct TCPStream {
    pub stream: TcpStream,
}

impl TCPStream {
    /*
        Read the 4 first bytes and use it to read the remaining bytes.
     */
    pub fn read(&mut self) -> io::Result<String> {
        let mut size = [0; 4];
        self.stream.read_exact(&mut size)?;
        let size = u32::from_be_bytes(size);
        let mut buffer = vec![0; size as usize];
        self.stream.read_exact(&mut buffer)?;
        Ok(String::from_utf8(buffer).unwrap())
    }

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