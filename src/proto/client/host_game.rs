extern crate protocol;
use crate::proto::AmongUsPacket;

#[derive(Protocol, Debug, PartialEq)]
pub struct HostGame{
    pub code: i32,
    pub map_own: u8
}