use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::prelude::*;
use std::path::Path;

use crate::boot_data::{AsU8Slice, BootData, BootDevice, DCD_DATA, ImageVectorTable};

pub fn append_ivt_header<P: AsRef<Path>>(
    input_path: P,
    output_path: P,
    entry_point: u32,
    boot_device: BootDevice,
) -> io::Result<()> {
    let mut input_file = File::open(&input_path)?;
    let mut output_file = File::create(&output_path)?;

    let input_absolute_path = std::fs::canonicalize(input_path).unwrap();
    let output_absolute_path = std::fs::canonicalize(output_path).unwrap();

    if input_absolute_path == output_absolute_path {
        return Err(io::Error::new(
            ErrorKind::AddrInUse,
            "Problem parsing arguments: input and output file cannot be same",
        ));
    }

    let ivt_data = ImageVectorTable::new(entry_point);
    let boot_data = BootData::new(
        entry_point,
        input_file.metadata().unwrap().len() as u32,
        boot_device,
    );

    // Offset
    write_padding(&mut output_file, boot_device as usize);
    // IVT
    output_file.write_all(ivt_data.as_u8_slice()).unwrap();
    // Boot data
    output_file.write_all(boot_data.as_u8_slice()).unwrap();
    // DCD
    output_file.write_all(DCD_DATA).unwrap();
    // Padding
    write_padding(&mut output_file, 0x9F0);
    // Application
    append_to_file(&mut input_file, &mut output_file);

    Ok(())
}

fn write_padding(f: &mut File, n: usize) {
    for _ in 0..n {
        f.write_all(&[0_u8]).unwrap();
    }
}

fn append_to_file(input: &mut File, output: &mut File) {
    let mut read_size = 1024;
    while read_size == 1024 {
        let mut buf = [0_u8; 1024];
        read_size = input.read(&mut buf).unwrap();
        output.write_all(&buf[0..read_size]).unwrap();
    }
}
