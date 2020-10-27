use crate::coding::{EncodeResult, Packet, DecodeResult, write_string};
use std::io::{Error, ErrorKind, Read, Write};
use byteorder::{ReadBytesExt, WriteBytesExt};

#[derive(Debug, Clone)]
pub enum Reason {
    None,
    GameFull,
    GameStarted,
    GameNotFound,
    CustomOld,
    OutdatedClient,
    BannedFromRoom,
    KickedFromRoom,
    Custom(String),
    InvalidUsername,
    Hacking,
    Force,
    BadConnection,
    GameNotFound2,
    ServerClosed,
    ServerOverloaded
}
impl Packet for Reason{
    fn decode(read: &mut dyn Read) -> DecodeResult<Reason> where
        Self: Sized {
        let r = read.read_u8()?;
        match r{
            0=> Ok(Reason::None),
            _ => Err(Error::new(ErrorKind::InvalidInput, format!("Invalid reason: {}", r)))
        }
    }

    fn handle(self: Box<Self>, write: &mut dyn Write) -> EncodeResult {
        let (t, str): (u8, Option<&String>) = match self.as_ref(){
            Reason::None => (0,None),
            Reason::GameFull => (1,None),
            Reason::GameStarted => (2,None),
            Reason::GameNotFound => (3,None),
            Reason::CustomOld => (4,None),
            Reason::OutdatedClient => (5,None),
            Reason::BannedFromRoom => (6,None),
            Reason::KickedFromRoom => (7,None),
            Reason::Custom(s) => (8,Some(s)),
            Reason::InvalidUsername => (9,None),
            Reason::Hacking => (10,None),
            Reason::Force => (0x10,None),
            Reason::BadConnection => (0x11,None),
            Reason::GameNotFound2 => (0x12,None),
            Reason::ServerClosed => (0x13,None),
            Reason::ServerOverloaded => (0x14,None)
        };
        write.write_u8(t);
        str.map_or((), |x| write_string(write,str.unwrap()).unwrap());
        Ok(())
    }
}