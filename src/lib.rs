use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::prelude::*;
use std::path::Path;

const IVT_DATA: &'static [u8] = include_bytes!("ivt.bin");

pub fn append_ivt_header<P: AsRef<Path>>(input_filepath: P, output_filepath: P) -> io::Result<()> {
    let mut input_file = File::open(&input_filepath)?;
    let mut output_file = File::create(&output_filepath)?;

    let input_filepath = std::fs::canonicalize(input_filepath).unwrap();
    let output_filepath = std::fs::canonicalize(output_filepath).unwrap();

    if input_filepath == output_filepath {
        return Err(io::Error::new(
            ErrorKind::AddrInUse,
            "Problem parsing arguments: input and output file cannot be same",
        ));
    }

    write_padding(&mut output_file, 0x400)?;
    output_file.write_all(IVT_DATA)?;
    write_padding(&mut output_file, 0x9F0)?;
    copy_to(&mut input_file, &mut output_file)?;

    Ok(())
}

fn write_padding(f: &mut File, n: usize) -> io::Result<()> {
    for _ in 0..n {
        f.write_all(&[0_u8])?;
    }
    Ok(())
}

fn copy_to(input: &mut File, output: &mut File) -> io::Result<()> {
    let mut read_size = 1024;
    while read_size == 1024 {
        let mut buf = [0_u8; 1024];
        read_size = input.read(&mut buf)?;
        output.write_all(&buf[0..read_size])?;
    }

    Ok(())
}