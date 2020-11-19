
#[cfg(test)]
pub mod tests {

    const V1CODES: [&str;4] = [
        "AAAA",
        "ABCD",
        "WXYZ",
        "MRCY"
    ];

    #[test]
    fn test_codes_to_num() {
        let v1nums: Vec<i32> = V1CODES.iter().map(|x| crate::codes::code_to_i32(x).unwrap()).collect();
        println!("{} = {:#08x}", V1CODES[0], v1nums[0]);
        assert_eq!(v1nums[0], 0x41414141);
        println!("{} = {:#08x}", V1CODES[1], v1nums[1]);
        assert_eq!(v1nums[1], (0x41424344 as i32).to_be());
        println!("{} = {:#08x}", V1CODES[2], v1nums[2]);
        assert_eq!(v1nums[2], (0x5758595A as i32).to_be());
        println!("{} = {:#08x}", V1CODES[3], v1nums[3]);
        assert_eq!(v1nums[3], (0x4D524359 as i32).to_be());
        let code = crate::codes::code_to_i32("FEVFOS").unwrap();
        println!("{:?}", code);
        //assert_eq!()
    }

    #[test]
    fn test_pack_varint(){
        //let x = 0xd209 as crate::r#type::varint::Varint;
        //println!("{:#02X?}",x.raw_bytes());
    }
}
