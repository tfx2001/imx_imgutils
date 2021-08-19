use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        Err("Usage: imx_generator <input_filepath> <output_filepath>")?;
    }

    imx_generator::append_ivt_header(&args[1], &args[2])?;

    Ok(())
}
