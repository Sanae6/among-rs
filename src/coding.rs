use std::io::{Read, Error, ErrorKind, Write};
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};

pub type DecodeResult<T> = Result<T, Error>;
pub type EncodeResult = Result<(), Error>;

pub fn read_bytes(read: &mut dyn Read, amt: usize) -> DecodeResult<Vec<u8>>{
    let mut buf: Vec<u8> = Vec::new();
    for _ in 0..amt {
        buf.push(read.read_u8()?);
    };
    Ok(buf)
}
pub fn read_varint(read: &mut dyn Read)-> DecodeResult<i32>{
    let mut read_more: bool = true;
    let mut output: i32 = 0;
    let mut shift: i32 = 0;
    while read_more {
        let mut b = read.read_u8()? as i32;
        if b >= 0x80{
            read_more = true;
            b = b ^ 0x80;
        } else {
            read_more = false;
        }

        output |= b << shift;
        shift += 7;
    }
    Ok(output)
}
pub fn read_string(read: &mut dyn Read)-> DecodeResult<String>{
    let amt = read_varint(read)? as usize;
    let mut buf = Vec::new();
    buf.resize(amt, 0);
    read.read(&mut *buf)?;
    String::from_utf8(buf)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))
}

pub fn write_bytes(write: &mut dyn Write, bytes: Vec<u8>) -> EncodeResult{
    write.write(&*bytes).map(|_| ())
}
pub fn write_varint(write: &mut dyn Write, val: i32) -> EncodeResult{
    let mut out: Vec<u8> = Vec::new();
    let mut valu = val;
    loop{
        let mut b = (valu & 0xff) as u8;
        if valu >= 0x80 {
            b = b | 0x80;
        }
        println!("{}",b);
        out.push(b);
        valu = valu >> 7;
        if valu <= 0 { break }
    }
    write.write(&*out).map(|x| ())
}
pub fn write_string(write: &mut dyn Write, val: &String)-> EncodeResult{
    println!("{:?}",val.len());
    write_varint(write, val.len() as i32).expect("failed to write varint");
    for x in val.as_bytes().iter(){
        write.write_u8(x-24)?;
    }
    Ok(())
}

pub trait Packet : Send{
    fn decode(read: &mut dyn Read)-> DecodeResult<Self>
        where
            Self : Sized;
    fn handle(self: Box<Self>, write: &mut dyn Write)-> EncodeResult
        where
            Self : Sized;
}