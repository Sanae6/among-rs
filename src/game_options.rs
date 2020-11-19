use crate::game_options::Map::{TheSkeld, MiraHq, Polus};
use crate::coding::{Packet, DecodeResult, EncodeResult};
use std::io::{Error, ErrorKind, Read, Write};
use byteorder::{WriteBytesExt, ReadBytesExt};

#[derive(Debug, PartialEq)]
pub struct GameOptions {
    len_useless: i32,
    version: u8,
    max_players: u8,
    language: Language,
    map: Map,
    player_speed: f32,
    crewmate_vision: f32,
    impostor_vision: f32,
    kill_cooldown: f32,
    common_tasks: u8,
    long_tasks: u8,
    short_tasks: u8,
    emergencies: i32,
    impostor_count: u8,
    kill_distance: u8,
    discussion_time: i32,
    voting_time: i32,
    is_default_settings: bool,
    emergency_cooldown: u8,
    confirm_ejects: bool,
    visual_tasks: bool
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
#[repr(u32)]
pub enum Language {
    Other = 1,
    Spanish = 2,
    Korean = 4,
    Russian = 8,
    Portuguese = 16,
    Arabic = 32,
    Filipino = 64,
    Polish = 128,
    English = 256
}

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Map {
    TheSkeld,
    MiraHq,
    Polus
}

impl Packet for Map {
    fn decode(read: &mut dyn Read) -> DecodeResult<Self> where
        Self: Sized {
        match read.read_u8()? {
            0 => Ok(TheSkeld),
            1 => Ok(MiraHq),
            2 => Ok(Polus),
            _ => Err(Error::new(ErrorKind::InvalidInput, ""))
        }
    }

    fn handle(self: Box<Self>, write: &mut dyn Write) -> EncodeResult {
        write.write_u8(match self.as_ref(){
            TheSkeld => 0,
            MiraHq => 1,
            Polus => 2
        })
    }
}
