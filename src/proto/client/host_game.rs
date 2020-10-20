extern crate protocol;

#[derive(Protocol, Debug, PartialEq)]
pub struct HostGame{
    pub code: i32,
    pub map_own: u8
}