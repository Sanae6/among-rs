use protocol::types::Integer;
use protocol::{Parcel, Settings, Error};
use std::io::{Read, Write, ErrorKind};
use protocol::hint::Hints;
use byteorder::{ReadBytesExt, BigEndian, WriteBytesExt};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Varint(i32);

impl Varint{
    pub fn new(int: i32) -> Varint{
        Varint{
            0: int
        }
    }
}

impl Parcel for Varint{
    const TYPE_NAME: &'static str = "Varint";

    fn read_field(read: &mut dyn Read, settings: &Settings, hints: &mut Hints) -> Result<Self, protocol::Error> {
        let mut read_more: bool = true;
        let mut output: i32 = 0;
        let mut shift: i32 = 0;
        while read_more {
            let mut b = read.read_u8().unwrap() as i32;
            if b >= 0x80{
                read_more = true;
                b = b ^ 0x80;
            } else {
                read_more = false;
            }

            output |= b << shift;
            shift += 7;
        }

        Ok(Varint::new(output))
    }

    fn write_field(&self, write: &mut dyn Write, settings: &Settings, hints: &mut Hints) -> Result<(), protocol::Error> {
        let mut n = self.0;
        loop {
            let mut b:u8 = (n & 0xff) as u8;
            if n >= 0x80 {
                b |= 0x80;
            }
            write.write_u8(b).unwrap();
            n >>= 7;
            if n <= 0 {
                break;
            }
        }
        Ok(())
    }
}

