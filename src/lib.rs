use std::io::{Error, ErrorKind, Read, Write};

pub mod codes;
pub mod packets;
pub mod reason;
mod coding;
mod tests;
pub(in crate) mod game_options;

pub use reason::Reason;
use byteorder::{ReadBytesExt, BigEndian, WriteBytesExt};
use crate::coding::{Packet, DecodeResult, EncodeResult};
use crate::packets::{Payload, HazelDataPacket, HazelPacket, HelloPayload, JoinGame};
use crate::packets::HostGameResponse;
use std::borrow::Borrow;

fn read_data_packet(read: &mut dyn Read, serverside: bool) -> DecodeResult<Payload>{
    match read.read_u8()? {
        0 => if serverside {
            Ok(Payload::HostGame())
        } else {
            Ok(Payload::HostGameResponse(HostGameResponse::decode(read)?))
        },
        1 => if serverside {
            Ok(Payload::JoinGame(JoinGame::decode(read)?))
        }else {
            Ok(Payload::HostGame())
        }
        _ => Err(Error::new(ErrorKind::InvalidInput,"Couldn't match a payload id!"))
    }
}

pub fn read_data(read: &mut dyn Read, serverside: bool) -> DecodeResult<HazelPacket>{
    match read.read_u8()? {
        0 => {
            Ok(HazelPacket::Normal(HazelDataPacket{
                nonce: 0,
                data: read_data_packet(read,serverside)?
            }))
        }
        1 => {
            Ok(HazelPacket::Reliable(HazelDataPacket{
                nonce: read.read_u16::<BigEndian>()?,
                data: read_data_packet(read,serverside)?
            }))
        }
        8 => Ok(HazelPacket::Hello(HazelDataPacket{
            nonce: read.read_u16::<BigEndian>()?,
            data: Payload::Hello(HelloPayload::decode(read)?)
        })),
        9 => Ok(HazelPacket::Disconnect(None)),
        10 => Ok(HazelPacket::Ack(read.read_u16::<BigEndian>()?)),
        12 => Ok(HazelPacket::Ping(read.read_u16::<BigEndian>()?)),
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid header control type"))
    }
}

fn write_data_packet(write: &mut dyn Write, payload: Payload, serverside: bool) -> EncodeResult{
    Ok(match payload{
        Payload::Hello(p) => {
            if serverside {
                unimplemented!()
            } else {
                HelloPayload::handle(Box::from(p), write)?;
            }
        }
        Payload::HostGame() => {

        }
        Payload::HostGameResponse(p) => {
            HostGameResponse::handle(Box::from(p), write)?;
        }
        Payload::JoinGame(p) => {
            JoinGame::handle(Box::from(p), write)?;
        }
        Payload::GameError(p) => {
            Reason::handle(Box::from(p), write)?;
        }
    })
}

pub fn write_reliable(write: &mut dyn Write, payload: Payload, nonce: u16, serverside: bool) -> EncodeResult{
    write.write_u8(1)?;
    write.write_u16::<BigEndian>(nonce)?;
    write_data_packet(write, payload, serverside);

    Ok(())
}

pub fn write_unreliable(write: &mut dyn Write, payload: Payload, serverside: bool) -> EncodeResult{
    write.write_u8(0)?;
    write_data_packet(write, payload, serverside);

    Ok(())
}

pub fn write_ack(write: &mut dyn Write, nonce: u16) -> EncodeResult{
    write.write_u8(10)?;
    write.write_u16::<BigEndian>(nonce)?;

    Ok(())
}

/**
Writes a disconnect packet.

Reason can be none, and will not add one if used.
*/
pub fn write_disconnect(write: &mut dyn Write, reason: Option<Reason>) -> EncodeResult{
    write.write_u8(9)?;
    if reason.is_some() {
        let r = reason.unwrap();
        Reason::handle(Box::from(r), write)?;
    }
    Ok(())
}