use staightforward_internet_checksum::checksum;
use internet_checksum::Checksum;

fn main() {
    // compare the result of straightforward internet checksum
    // and that from the external crate

    // straightforward
    let buf: [u16; 3] = [0xf000, 0xf200, 0xf321];
    let buf = &buf[..];
    let sum1 = checksum(buf);

    // external    
    let mut checksum = Checksum::new();
    checksum.add_bytes(&[0xf0, 0x00, 0xf2, 0x00, 0xf3, 0x21][..]);
    let sum2 = checksum.checksum();
    let sum2 = (sum2[0] as u16)* 256 + (sum2[1] as u16); 
    
    assert_eq!(sum1, sum2);    
}
