use std::{env, fs::File, io::Read};

const BYTES_PER_LINE: usize = 16;

fn main() {
    let file_name_arg = env::args().nth(1).expect("File name needed");
    let mut file = File::open(&file_name_arg).expect("Unable to open file");
    let mut pos = 0;
    let mut buffer = [0; BYTES_PER_LINE];

    while matches!(file.read_exact(&mut buffer), Ok(())) {
        print!("[0x{pos:08x}] ");
        for byte in &buffer {
            match *byte {
                0x00 => print!(".  "),
                0xff => print!("## "),
                _ => print!("{byte:02x} "),
            }
        }
        println!();
        pos += BYTES_PER_LINE;
    }
}
