pub fn checksum(buf: &[u16]) -> u16 {
    let mut sum: u32 = 0;
    for word in buf.iter() {
        sum += *word as u32;
        // If carry occured, add 1 to the least significant bit
        if sum & 0xffff0000 > 0 {
            sum &= 0xffff;
            sum += 1;
        }
    }   
    let rlt = (sum & 0xffff) as u16;
    // return the complement
    !rlt
}