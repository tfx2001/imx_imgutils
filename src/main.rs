use std::error;

use structopt::StructOpt;

use imx_imgutils::CliOpt;

fn main() -> Result<(), Box<dyn error::Error>> {
    let opt = CliOpt::from_args();

    imx_imgutils::append_ivt_header(
        &opt.input_path,
        &opt.output_path,
        opt.entry_point,
        opt.boot_device,
    )?;

    Ok(())
}
