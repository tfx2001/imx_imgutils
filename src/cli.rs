use std::num::ParseIntError;
use std::path::PathBuf;

use structopt::StructOpt;

use crate::boot_data::BootDevice;

fn parse_hex(s: &str) -> Result<u32, ParseIntError> {
    u32::from_str_radix(s.trim_start_matches("0x"), 16)
}

fn parse_boot_device(device: &str) -> Result<BootDevice, &str> {
    match device {
        "sd" => Ok(BootDevice::SD),
        "ram" => Ok(BootDevice::RAM),
        _ => Err("Input error, allowed values: sd, ram"),
    }
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "imx_generator",
    about = "Add boot data to image for ALIENTEK Alpha i.MX board."
)]
pub struct CLIOpt {
    #[structopt(name = "input", help = "Path to input file", parse(from_os_str))]
    pub input_path: PathBuf,
    #[structopt(name = "output", help = "Path to output file", parse(from_os_str))]
    pub output_path: PathBuf,
    #[structopt(short, long, help = "Entry point of application", default_value = "0x87800000", parse(try_from_str = parse_hex))]
    pub entry_point: u32,
    #[structopt(name = "boot", short, long, help = "Boot device, possible values: ram, sd", default_value = "ram", parse(try_from_str = parse_boot_device))]
    pub boot_device: BootDevice,
}
