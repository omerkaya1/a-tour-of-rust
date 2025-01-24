#![allow(dead_code)]
#[repr(C)]
#[derive(bytemuck::Zeroable, bytemuck::Pod, Clone, Copy, Debug)]
struct Data {
    num: u16,
    tag: [u8; 8],
}

fn main() -> anyhow::Result<()> {
    let some = vec![
        Data{num: 1, tag: *b"Hello   ",},
        Data{num: 2, tag: *b"data!   ",},
    ];

    let bytes: &[u8] = bytemuck::cast_slice(&some);
    std::fs::write("data.bin", bytes)?;

    // read the data from file
    let b = std::fs::read("data.bin").unwrap();
    let d: &[Data] = bytemuck::cast_slice(&b);

    println!("{d:?}");

    Ok(())
}
