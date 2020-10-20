extern crate protocol;
extern crate protocol_derive;

#[derive(Protocol, Debug, PartialEq, AmongUsPacket)]
pub struct JoinGameC2S{
    pub code: i32,
    pub map_own: u8
}