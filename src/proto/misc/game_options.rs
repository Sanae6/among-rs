use crate::util::varint::Varint;
extern crate protocol;
use protocol_derive::*;

#[derive(Protocol, Debug, PartialEq)]
pub struct GameOptions{
    len_useless: Varint,
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

#[derive(Protocol, Debug, PartialEq)]
#[protocol(discriminant = "integer")]
#[repr(u32)]
pub enum Language{
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

#[derive(Protocol, Debug, PartialEq)]
#[protocol(discriminant = "integer")]
#[repr(u8)]
pub enum Map {
    TheSkeld,
    MiraHq,
    Polus
}