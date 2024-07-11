use std::error::Error;
use clap::Parser;
use resampler::resamplers::{core, do_algorithm, file_use};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The source file of the samples
    source: String,

    /// The input side of the resampling ratio
    input_ratio: String,

    /// The output side of the resampling ratio
    output_ratio: String,

    #[arg(short, long, default_value ="terminal")]
    /// The destination file of the samples
    dest: String,
    
    #[arg(short, long, default_value ="lerp")]
    interpolation_type: String,

    #[arg(short, long, default_value ="f32")]
    precision: String,
}


fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut resampler = core::Resampler::from_args(
        args.source,
        args.dest,
        args.input_ratio.parse::<f32>().unwrap(),
        args.output_ratio.parse::<f32>().unwrap(),
    );
    
    resampler.data_get()?;

    match args.interpolation_type.to_lowercase().as_str() {
        "lerp" | "linear" | "l" => resampler.do_lerp(),
        "piecewise_constant" | "piecewise" | "piece" | "p"
            => resampler.do_piecewise_constant(),
        "tangentless_hermite" | "th" => resampler.do_tangentless_hermite(),
        "catmull_rom" | "cat" | "cr" => resampler.do_catmull_rom_spline(),
        _ => panic!("Interpolation type not found!"),
    }

    resampler.data_put()?;

    Ok(())
}
