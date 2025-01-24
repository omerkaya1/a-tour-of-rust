#![allow(dead_code)]

struct OneByte {
    a: u8,
}

struct TwoBytes {
    a: u16,
}

// #[repr(C)] - allows for making sure that the order is always preserved
// #[no_mangle] - allows for making sure that the names are preserved at all times!
#[repr(packed)] // this will make sure it's really 3 bytes; still - respect the struct allignment!
struct ThreeBytes { // sure!!!
    a: u16,
    b: u8,
}

struct FourBytes {
    a: u32,
}

fn main() {
    println!("{}", std::mem::size_of::<OneByte>());
    println!("{}", std::mem::size_of::<TwoBytes>());
    println!("{}", std::mem::size_of::<ThreeBytes>());
    println!("{}", std::mem::size_of::<FourBytes>());
}
