use io;

pub fn set_position(index: u16) {
    let mut video_port = io::Port::open(io::port::Address(0x3d4));
    let mut other_port = io::Port::open(io::port::Address(0x3d5));

    // cursor LOW port to vga INDEX register
    video_port.write_u8(0x0f);
    other_port.write_u8((index & 0xff) as u8);

    // cursor HIGH port to vga INDEX register
    video_port.write_u8(0x0e);
    other_port.write_u8(((index >> 8) & 0xff) as u8);
}

