extern crate protocol;
extern crate protocol_derive;

#[derive(Protocol, Debug, PartialEq)]
pub struct JoinGameC2S{
    pub code: i32,
    pub map_own: u8
}

#[derive(Protocol, Debug, PartialEq)]
pub struct JoinGameS2C{
    pub dc_reason: u8
}