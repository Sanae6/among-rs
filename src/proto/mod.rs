pub mod client;
pub mod server;
pub mod misc;
pub mod hazel;

use protocol::Parcel;
pub trait AmongUsPacket: Parcel{}