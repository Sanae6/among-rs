extern crate protocol;
extern crate protocol_derive;
use crate::proto::AmongUsPacket;

#[derive(Protocol, Debug, PartialEq, AmongUsPacket)]
pub struct JoinGameError{
    pub dc_reason: u8
}