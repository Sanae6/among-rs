use std::io::{Read, Write, Cursor};
use protocol::{HighLevel, Settings, Parcel};
use protocol::hint::Hints;
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};
use crate::proto::hazel::HazelPacket::{Normal, Reliable, Hello, Disconnect, Ack, Ping, Unknown};

pub struct RawHazelPacket{
    op: u8,
    data: Vec<u8>
}

impl Parcel for RawHazelPacket{
    const TYPE_NAME: &'static str = "rhp";

    fn read_field(read: &mut dyn Read, _settings: &Settings, _hints: &mut Hints) -> Result<Self, protocol::Error> {
        let op = read.read_u8()?;
        let mut data: Vec<u8> = Vec::new();
        Ok(RawHazelPacket{
            op,
            data
        })
    }

    fn write_field(&self, write: &mut dyn Write, _settings: &Settings, _hints: &mut Hints) -> Result<(), protocol::Error> {
        write.write_u8(self.op);
        write.write_all(&*self.data);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum HazelPacket{
    Normal {
        data: Vec<u8>
    },
    Reliable {
        id: u16,
        data: Vec<u8>
    },
    Hello {
        id: u16,
        data: Vec<u8>
    },
    Disconnect,
    Ack {
        id: u16
    },
    Ping {
        id: u16
    },
    Unknown
}

impl protocol::HighLevel for HazelPacket{
    type LowLevel = RawHazelPacket;

    fn into_low_level(self) -> Self::LowLevel {
        Self::LowLevel{
            op:match self {
                HazelPacket::Normal {..} => 0,
                HazelPacket::Reliable {..} => 1,
                HazelPacket::Hello {..} => 8,
                HazelPacket::Disconnect {..} => 9,
                HazelPacket::Ack {..} => 10,
                HazelPacket::Ping {..} => 12,
                HazelPacket::Unknown => 9//unknown = disconnect!!!
            },
            data: self.raw_bytes(&Settings::default()).unwrap()
        }
    }

    fn from_low_level(value: Self::LowLevel, _subsequent_reader: &mut dyn Read, _settings: &Settings, _hints: &mut Hints) -> Result<Self, protocol::Error> {
        let out = match value.op{
            0 => Normal { data: value.data },
            1 => {
                let mut cur = Cursor::new(&value.data);
                let id = cur.read_u16::<BigEndian>()?.to_be();
                let mut data = Vec::new();
                cur.read_to_end(&mut data);
                Reliable { id, data }
            },
            8 => {
                let mut cur = Cursor::new(&value.data);
                let id = cur.read_u16::<BigEndian>()?;
                let mut data = Vec::new();
                cur.read_to_end(&mut data);
                Hello { id, data }
            },
            9 => Disconnect,
            10 => Ack { id: Cursor::new(&value.data).read_u16::<BigEndian>()? },
            12=> Ping { id: Cursor::new(&value.data).read_u16::<BigEndian>()? },
            _ => Unknown
        };

        Ok(out)
    }
}