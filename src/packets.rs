use byteorder::{ReadBytesExt, BigEndian, WriteBytesExt};

use crate::coding::{DecodeResult, EncodeResult, Packet, write_string};
use crate::Reason;
use std::io::{Read, Write};

#[derive(Debug, Clone)]
pub enum HazelPacket {
    Normal(HazelDataPacket),
    Reliable(HazelDataPacket),
    Hello(HazelDataPacket),
    Ack(u16),
    Ping(u16),
    Disconnect(Option<HazelDisconnectPacket>)
}

#[derive(Debug, Clone)]
pub struct HazelDisconnectPacket {}

#[derive(Debug, Clone)]
pub struct HazelDataPacket {
    //zero if normal
    pub nonce: u16,
    pub data: Payload
}

#[derive(Debug, Clone)]
pub enum Payload {
    Hello(HelloPayload),
    HostGame(),
    HostGameResponse(HostGameResponse),
    JoinGame(JoinGame),
    GameError(Reason)
}

#[derive(Debug, Clone)]
pub struct HelloPayload {
    pub name: String
}

impl Packet for HelloPayload {
    fn decode(read: &mut dyn Read) -> DecodeResult<Self> {
        read.read_u8()?; //hazel version
        read.read_u32::<BigEndian>()?;
        Ok(HelloPayload{
            name: crate::coding::read_string(read)?
        })
    }

    fn handle(self: Box<Self>, write: &mut dyn Write) -> EncodeResult {
        write.write_u8(0)?;
        write.write_u32::<BigEndian>(0x46_d2_02_03)?;
        write_string(write, &self.name)
    }
}

#[derive(Debug, Clone)]
pub struct HostGameResponse {
    code: i32
}

impl Packet for HostGameResponse{
    fn decode(read: &mut dyn Read) -> DecodeResult<Self> {
        Ok(HostGameResponse {
            code: read.read_i32::<BigEndian>()?
        })
    }

    fn handle(self: Box<Self>, write: &mut dyn Write) -> EncodeResult {
        write.write_i32::<BigEndian>(self.code)
    }
}

#[derive(Debug, Clone)]
pub struct JoinGame {
    code: i32
}

impl Packet for JoinGame {
    fn decode(read: &mut dyn Read) -> DecodeResult<Self> {
        let code = read.read_i32::<BigEndian>()?;
        read.read_u8()?;
        Ok(JoinGame{ code })
    }

    fn handle(self: Box<Self>, write: &mut dyn Write) -> EncodeResult {
        write.write_i32::<BigEndian>(self.code)?;
        write.write_u8(0)?;
        Ok(())
    }
}
