const V2MAP: [u8;26] = [
    0x19,
    0x15,
    0x13,
    0x0A,
    0x08,
    0x0B,
    0x0C,
    0x0D,
    0x16,
    0x0F,
    0x10,
    0x06,
    0x18,
    0x17,
    0x12,
    0x07,
    0x00,
    0x03,
    0x09,
    0x04,
    0x0E,
    0x14,
    0x01,
    0x02,
    0x05,
    0x11
];
fn remap(char: i32)->i32{
    V2MAP[char as usize - 65 as usize] as i32
}

pub fn code_to_i32(code: &str) -> Result<i32,&str>{
    if code.len() == 4 {
        let x: Vec<i32> = code.as_bytes().iter().map(|x|*x as i32).collect();
        Ok((x[0] | ((x[1] | ((x[2] | (x[3] << 8)) << 8)) << 8)) as i32)
    }
    else if code.len() == 6 {
        let x: Vec<i32> = code.as_bytes().iter().map(|x|*x as i32).collect();
        let one: i32 = (remap(x[0]) + 26 * remap(x[1])) & 0x3FF;
        let two: i32 = remap(x[2]) + 26 * (remap(x[3]) + 26 * (remap(x[4]) + 26 * remap(x[5])));
        Ok(one | ((two << 10) & 0x3FFFFC00) | i32::MIN)
    }else {
        Err("Code must be length of 4 (V1) or 6 (V2)")
    }

}

/*pub fn i32_to_code_v2() -> &str{

    ""
}*/