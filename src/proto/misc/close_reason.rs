#[derive(Protocol, Debug, Clone)]
#[protocol(discriminant = "integer")]
#[repr(u8)]
pub enum Reason {
    None,
    GameFull,
    GameStarted,
    GameNotFound,
    CustomOld,
    OutdatedClient,
    BannedFromRoom,
    KickedFromRoom,
    Custom,
    InvalidUsername,
    Hacking,
    Force = 0x10,
    BadConnection,
    GameNotFound2,
    ServerClosed,
    ServerOverloaded,
}